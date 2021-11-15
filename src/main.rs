// Copied from the docs
use oso::{Oso, PolarClass};

#[derive(Clone, PolarClass)]
struct User {
    #[polar(attribute)]
    pub username: String,
}

impl User {
    fn superuser() -> Vec<String> {
        return vec!["alice".to_string(), "charlie".to_string()];
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut oso = Oso::new();

    oso.register_class(
        User::get_polar_class_builder()
            .add_class_method("superusers", User::superuser)
            .build(),
    )?;

    oso.load_str(
        r#"allow(actor: User, _action, _resource) if
                        actor.username.ends_with("example.com");"#,
    )?;

    let user1 = User {
        username: "alice@example.com".to_owned(),
    };
    let user2 = User {
        username: "alice@xample.com".to_owned(),
    };
    assert!(oso.is_allowed(user1, "foo", "bar")?);
    assert!(!oso.is_allowed(user2, "foo", "bar")?);
    Ok(())
}
