use crate::config::Config;
use openssl::asn1::Asn1Time;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::symm::Cipher;
use openssl::x509::{X509NameBuilder, X509Req, X509ReqBuilder, X509};
use shared::util::types::BasicResult;
use std::path::Path;
use tokio::fs;

pub struct Certificate {
    key_pair: PKey<Private>,
    cert: Option<X509>,
}

impl Certificate {
    pub fn expires_in_secs(&self) -> BasicResult<Option<u64>> {
        let now = Asn1Time::days_from_now(0)?;
        let now = now.as_ref().to_owned();

        if let Some(c) = &self.cert {
            Ok(Some(c.not_after().diff(&now)?.secs as u64))
        } else {
            Ok(None)
        }
    }

    pub fn set_certificate(&mut self, cert: X509) {
        self.cert = Some(cert);
    }

    pub async fn load(dir: String, passphrase: Option<&String>) -> BasicResult<Self> {
        let dir = Path::new(&dir);
        let private = fs::read(dir.join("private.pem")).await?;
        let public = fs::read(dir.join("public.pem")).await?;
        let cert = if let Ok(cert) = fs::read(dir.join("cert.pem")).await {
            Some(X509::from_pem(cert.as_slice())?)
        } else {
            None
        };

        let key_pair = if let Some(pass) = passphrase {
            PKey::private_key_from_pem_passphrase(private.as_slice(), pass.as_bytes())?
        } else {
            PKey::private_key_from_pem(private.as_slice())?
        };
        let public_key = PKey::public_key_from_pem(public.as_slice())?;

        if key_pair.public_eq(&public_key) {
            Ok(Self { key_pair, cert })
        } else {
            Err("Public and private keys do not match".into())
        }
    }

    pub fn generate() -> BasicResult<Self> {
        let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?;
        let private = EcKey::generate(&group)?;
        let key_pair = PKey::from_ec_key(private)?;

        Ok(Self {
            key_pair,
            cert: None,
        })
    }

    pub async fn store(&self, out_dir: String, passphrase: Option<&String>) -> BasicResult<()> {
        let private = if let Some(pass) = passphrase {
            self.key_pair
                .private_key_to_pem_pkcs8_passphrase(Cipher::aes_256_cbc(), pass.as_bytes())?
        } else {
            self.key_pair.private_key_to_pem_pkcs8()?
        };

        let public = self.key_pair.public_key_to_pem()?;

        let out = Path::new(&out_dir);
        if !fs::try_exists(out).await? {
            fs::create_dir_all(out).await?;
        }

        fs::write(out.join("private.pem"), private).await?;
        fs::write(out.join("public.pem"), public).await?;

        if let Some(cert) = self.cert.as_ref() {
            let cert = cert.to_pem()?;
            fs::write(out.join("cert.pem"), cert).await?;
        }

        Ok(())
    }

    pub fn get_signing_request(&self, config: &Config) -> BasicResult<X509Req> {
        let mut req_builder = X509ReqBuilder::new()?;
        req_builder.set_pubkey(self.key_pair.as_ref())?;

        let mut x509_name = X509NameBuilder::new()?;
        if let Some(org) = &config.cert_organization {
            x509_name.append_entry_by_text("O", org)?;
        }
        if let Some(org_unit) = &config.cert_organization_unit {
            x509_name.append_entry_by_text("OU", org_unit)?;
        }
        if let Some(country) = &config.cert_country {
            x509_name.append_entry_by_text("C", country)?;
        }
        if let Some(state) = &config.cert_state {
            x509_name.append_entry_by_text("ST", state)?;
        }
        if let Some(locality) = &config.cert_locality {
            x509_name.append_entry_by_text("L", locality)?;
        }
        if let Some(email) = &config.cert_email {
            x509_name.append_entry_by_text("E", email)?;
        }

        x509_name.append_entry_by_text("CN", &config.cert_common_name)?;
        let x509_name = x509_name.build();
        req_builder.set_subject_name(&x509_name)?;

        req_builder.sign(self.key_pair.as_ref(), MessageDigest::sha256())?;
        Ok(req_builder.build())
    }
}
