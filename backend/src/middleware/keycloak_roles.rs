pub trait KeycloakRoles {
    fn get_roles() -> Vec<String>;

    fn roles_match(roles: &Vec<String>) -> bool {
        for role in Self::get_roles() {
            if !roles.contains(&role) {
                return false;
            }
        }

        true
    }
}

pub struct NoRoles;

impl KeycloakRoles for NoRoles {
    fn get_roles() -> Vec<String> {
        vec![]
    }
}

pub struct AdminRole;

impl KeycloakRoles for AdminRole {
    fn get_roles() -> Vec<String> {
        vec!["admin".to_string()]
    }
}
