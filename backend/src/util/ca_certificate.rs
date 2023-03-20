#![allow(clippy::uninlined_format_args)]

use crate::entity::certificate;
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectAlternativeName,
    SubjectKeyIdentifier,
};
use openssl::x509::{X509NameBuilder, X509Req, X509};
use sea_orm::prelude::DateTimeWithTimeZone;
use shared::util::types::BasicResult;
use std::error::Error;

pub struct CACertificate {
    cert: X509,
    key_pair: PKey<Private>,
}

impl CACertificate {
    /// Make a CA certificate and private key
    pub fn generate() -> BasicResult<Self> {
        let rsa = Rsa::generate(2048)?;
        let key_pair = PKey::from_rsa(rsa)?;

        let mut x509_name = X509NameBuilder::new()?;
        x509_name.append_entry_by_text("C", "US")?;
        x509_name.append_entry_by_text("ST", "TX")?;
        x509_name.append_entry_by_text("O", "Some CA organization")?;
        x509_name.append_entry_by_text("CN", "ca test")?;
        let x509_name = x509_name.build();

        let mut cert_builder = X509::builder()?;
        cert_builder.set_version(2)?;
        let serial_number = {
            let mut serial = BigNum::new()?;
            serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
            serial.to_asn1_integer()?
        };
        cert_builder.set_serial_number(&serial_number)?;
        cert_builder.set_subject_name(&x509_name)?;
        cert_builder.set_issuer_name(&x509_name)?;
        cert_builder.set_pubkey(&key_pair)?;
        let not_before = Asn1Time::days_from_now(0)?;
        cert_builder.set_not_before(&not_before)?;
        let not_after = Asn1Time::days_from_now(365)?;
        cert_builder.set_not_after(&not_after)?;

        cert_builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;
        cert_builder.append_extension(
            KeyUsage::new()
                .critical()
                .key_cert_sign()
                .crl_sign()
                .build()?,
        )?;

        let subject_key_identifier =
            SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
        cert_builder.append_extension(subject_key_identifier)?;

        cert_builder.sign(&key_pair, MessageDigest::sha256())?;
        let cert = cert_builder.build();

        Ok(Self { cert, key_pair })
    }

    /// Make a certificate and private key signed by the given CA cert and private key
    pub fn sign_request(
        &self,
        req: &X509Req,
        alt_names: &Option<Vec<String>>,
    ) -> BasicResult<X509> {
        let mut cert_builder = X509::builder()?;
        cert_builder.set_version(2)?;
        let serial_number = {
            let mut serial = BigNum::new()?;
            serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
            serial.to_asn1_integer()?
        };
        cert_builder.set_serial_number(&serial_number)?;
        cert_builder.set_subject_name(req.subject_name())?;
        cert_builder.set_issuer_name(self.cert.subject_name())?;
        cert_builder.set_pubkey(&self.key_pair)?;
        let not_before = Asn1Time::days_from_now(0)?;
        cert_builder.set_not_before(&not_before)?;
        let not_after = Asn1Time::days_from_now(365)?;
        cert_builder.set_not_after(&not_after)?;

        cert_builder.append_extension(BasicConstraints::new().build()?)?;

        cert_builder.append_extension(
            KeyUsage::new()
                .critical()
                .non_repudiation()
                .digital_signature()
                .key_encipherment()
                .build()?,
        )?;

        let subject_key_identifier = SubjectKeyIdentifier::new()
            .build(&cert_builder.x509v3_context(Some(&self.cert), None))?;
        cert_builder.append_extension(subject_key_identifier)?;

        let auth_key_identifier = AuthorityKeyIdentifier::new()
            .keyid(false)
            .issuer(false)
            .build(&cert_builder.x509v3_context(Some(&self.cert), None))?;
        cert_builder.append_extension(auth_key_identifier)?;

        if let Some(alt_names) = alt_names {
            if !alt_names.is_empty() {
                let mut subject_alt_name = SubjectAlternativeName::new();
                for alt_name in alt_names {
                    subject_alt_name.dns(alt_name);
                }

                let built =
                    subject_alt_name.build(&cert_builder.x509v3_context(Some(&self.cert), None))?;
                cert_builder.append_extension(built)?;
            }
        }

        cert_builder.sign(&self.key_pair, MessageDigest::sha256())?;
        Ok(cert_builder.build())
    }

    pub fn cert_as_pem(&self) -> BasicResult<Vec<u8>> {
        self.cert.to_pem().map_err(|e| e.into())
    }

    pub fn key_pair_as_pem(&self) -> BasicResult<Vec<u8>> {
        self.key_pair
            .private_key_to_pem_pkcs8()
            .map_err(|e| e.into())
    }

    pub fn valid_until(&self) -> BasicResult<DateTimeWithTimeZone> {
        let not_after = self.cert.not_after();
        let not_after = not_after.to_string() + " +0000";
        DateTimeWithTimeZone::parse_from_str(&not_after, "%b %d %H:%M:%S %Y GMT %z")
            .map_err(|e| e.into())
    }
}

impl TryFrom<certificate::Model> for CACertificate {
    type Error = Box<dyn Error>;

    fn try_from(value: certificate::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            cert: X509::from_pem(&value.public)?,
            key_pair: PKey::private_key_from_pem(
                &value
                    .private
                    .ok_or("The private key is not set".to_string())?,
            )?,
        })
    }
}
