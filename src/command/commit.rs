use anyhow::{Context, Result};
use clap::Args;
use git2::Repository;

const ABOUT_COMMIT: &str = "Commit Command";
const LONG_ABOUT_COMMIT: &str = "Commit Command Long Description";

#[derive(Debug, Args)]
#[command(about = ABOUT_COMMIT, long_about = LONG_ABOUT_COMMIT)]
pub(crate) struct CommitArgs {
    /// Commit message (required)
    #[arg(short, long)]
    message: Option<String>,
}

impl CommitArgs {
    pub fn run(&self) -> Result<()> {
        // コミットメッセージが指定されていなければエラー
        // TODO: メッセージの入力とプレフィックスの選択を実装したい
        let commit_message = self.message.as_deref().ok_or_else(|| {
            anyhow::anyhow!(
                "No commit message specified. Please provide a message using the -m option"
            )
        })?;

        // 現在のディレクトリから.gitディレクトリを探索し、リポジトリのルートを自動検出する
        let repo = Repository::discover(".")
            .context("Could not find the repository root. A `.git` directory must exist in the current directory or one of its parent directories")?;

        // Git の設定に基づいた署名情報を取得
        let sig = repo
            .signature()
            .context("Could not retrieve Git signature information. Please check the user.name and user.email settings.")?;

        // インデックス（ステージされた変更）を取得する
        let mut index = repo
            .index()
            .context("Could not retrieve the repository index")?;

        // インデックスからツリーオブジェクトを作成する
        let tree_oid = index
            .write_tree()
            .context("Could not create a tree from the index")?;
        let tree = repo
            .find_tree(tree_oid)
            .context("Could not retrieve the created tree.")?;

        // HEAD の存在で通常コミットか初回コミットかを分岐する
        let commit_oid = if let Ok(head) = repo.head() {
            // HEAD から最新のコミットを取得
            let parent_commit = head
                .peel_to_commit()
                .context("Could not convert HEAD to a commit")?;
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                commit_message,
                &tree,
                &[&parent_commit],
            )
            .context("Failed to create a commit")?
        } else {
            // HEAD が存在しない場合は初回コミット
            repo.commit(Some("HEAD"), &sig, &sig, commit_message, &tree, &[])
                .context("Could not retrieve the repository index")?
        };

        println!("Commit has been created: {}", commit_oid);
        Ok(())
    }
}
