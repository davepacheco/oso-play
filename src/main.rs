use oso::{Oso, PolarClass};

#[derive(Clone, Copy, PolarClass)]
struct User {
    #[polar(attribute)]
    pub username: &'static str,
}

#[derive(Clone, Copy)]
struct Folder {
    pub name: &'static str,
}

impl oso::PolarClass for Folder {
    fn get_polar_class() -> oso::Class {
        Self::get_polar_class_builder()
            .set_equality_check(|f1, f2| f1.name == f2.name)
            .build()
    }
}

#[derive(Clone, Copy, PolarClass)]
struct Document {
    #[polar(attribute)]
    folder: Folder,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut oso = Oso::new();

    oso.register_class(User::get_polar_class())?;
    oso.register_class(Folder::get_polar_class())?;
    oso.register_class(Document::get_polar_class())?;

    oso.load_str(
        r#"
        actor User {}

        resource Folder {
            permissions = [ "edit" ];
            roles = [ "admin" ];
            "edit" if "admin";
        }

        resource Document {
            permissions = [ "edit" ];
            roles = [ "admin" ];
            "edit" if "admin";

            relations = { parent: Folder };
            "admin" if "admin" on "parent";
        }

        allow(actor: Actor, action: String, resource: Resource) if
            has_permission(actor, action, resource);
        has_relation(folder: Folder, "parent", doc: Document)
            if folder = doc.folder;
        has_role(actor: User, "admin", _resource: Document)
            if actor.username = "doc_admin";
        has_role(actor: User, "admin", _resource: Folder)
            if actor.username = "folder_admin";
    "#,
    )?;

    let doc_admin = User {
        username: "doc_admin",
    };
    let folder_admin = User {
        username: "folder_admin",
    };
    let folder = Folder { name: "the_folder" };
    let doc = Document { folder };
    // Easy cases
    assert!(oso.is_allowed(folder_admin, "edit", folder).unwrap());
    assert!(!oso.is_allowed(doc_admin, "edit", folder).unwrap());
    assert!(oso.is_allowed(doc_admin, "edit", doc).unwrap());
    // Leverage the relationship
    assert!(oso.is_allowed(folder_admin, "edit", doc).unwrap());
    Ok(())
}
