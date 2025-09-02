use casbin::{CoreApi, Enforcer, FileAdapter, DefaultModel as Model};
use anyhow::Result;

pub struct CheckPermissionArgs<'a> {
    pub user: &'a str,
    pub resource: &'a str,
    pub action: &'a str,
    pub model_path: &'a str,
    pub policy_path: &'a str,
}

pub async fn check_permission(args: CheckPermissionArgs<'_>) -> Result<bool> {
    let m = Model::from_file(args.model_path).await?;
    let a = FileAdapter::new(args.policy_path.to_string());
    let e = Enforcer::new(m, a).await?;

    Ok(e.enforce((args.user, args.resource, args.action))?)
}

#[cfg(test)]
mod tests {
    use super::{check_permission, CheckPermissionArgs};

    #[tokio::test]
    async fn test_check_permission() -> anyhow::Result<()> {
        let model_path = "src/model.conf";
        let policy_path = "src/policy.csv";

        // Alice can read data1
        assert!(check_permission(CheckPermissionArgs {
            user: "alice",
            resource: "data1",
            action: "read",
            model_path,
            policy_path,
        }).await?);
        // Alice cannot write data1
        assert!(!check_permission(CheckPermissionArgs {
            user: "alice",
            resource: "data1",
            action: "write",
            model_path,
            policy_path,
        }).await?);

        // Bob can write data2
        assert!(check_permission(CheckPermissionArgs {
            user: "bob",
            resource: "data2",
            action: "write",
            model_path,
            policy_path,
        }).await?);
        // Bob cannot read data2
        assert!(!check_permission(CheckPermissionArgs {
            user: "bob",
            resource: "data2",
            action: "read",
            model_path,
            policy_path,
        }).await?);

        // Charlie cannot do anything
        assert!(!check_permission(CheckPermissionArgs {
            user: "charlie",
            resource: "data1",
            action: "read",
            model_path,
            policy_path,
        }).await?);

        Ok(())
    }
}
