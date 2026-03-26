use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::{keypair::read_keypair_file, Signer};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn temp_home() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("spl_forge_smoke_{}_{}", std::process::id(), nanos));
    fs::create_dir_all(&dir).expect("failed to create temp home");
    dir
}

fn run_cmd(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_spl-forge"))
        .args(args)
        .env("HOME", home)
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to run spl-forge")
}

fn assert_ok(output: &Output, args: &[&str]) {
    if !output.status.success() {
        panic!(
            "command failed: spl-forge {}\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn assert_fail(output: &Output, args: &[&str]) {
    if output.status.success() {
        panic!(
            "command unexpectedly succeeded: spl-forge {}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn validator_ready() -> bool {
    let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());
    rpc.get_latest_blockhash().is_ok()
}

fn wait_for_airdrop_balance(rpc: &RpcClient, pubkey: &solana_sdk::pubkey::Pubkey) {
    for _ in 0..30 {
        let bal = rpc.get_balance(pubkey).unwrap_or(0);
        if bal > 0 {
            return;
        }
        thread::sleep(Duration::from_millis(500));
    }
    panic!("airdrop did not finalize in time");
}

#[test]
fn config_and_help_smoke() {
    let home = temp_home();

    let out = run_cmd(&home, &["help"]);
    assert_ok(&out, &["help"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("SPL-FORGE"));

    let out = run_cmd(&home, &["config", "set", "devnet"]);
    assert_ok(&out, &["config", "set", "devnet"]);

    let out = run_cmd(&home, &["config", "set", "mainnet"]);
    assert_ok(&out, &["config", "set", "mainnet"]);

    let out = run_cmd(&home, &["config", "set", "localhost"]);
    assert_ok(&out, &["config", "set", "localhost"]);

    let out = run_cmd(&home, &["config", "set", "invalidnet"]);
    assert_fail(&out, &["config", "set", "invalidnet"]);

    let out = run_cmd(&home, &["config", "get"]);
    assert_ok(&out, &["config", "get"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("http://127.0.0.1:8899"));

    let out = run_cmd(&home, &["wallet", "address"]);
    assert_ok(&out, &["wallet", "address"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Address:"));

    let out = run_cmd(&home, &["wallet", "balance", "--address", "invalid_pubkey"]);
    assert_fail(&out, &["wallet", "balance", "--address", "invalid_pubkey"]);

    let out = run_cmd(&home, &["config", "reset"]);
    assert_ok(&out, &["config", "reset"]);
}

#[test]
fn create_command_smoke_localnet() {
    if !validator_ready() {
        eprintln!("skipping localnet create smoke: validator not reachable at 127.0.0.1:8899");
        return;
    }

    let home = temp_home();

    // Make sure config/keypair exist and localnet is selected.
    let out = run_cmd(&home, &["config", "set", "localhost"]);
    assert_ok(&out, &["config", "set", "localhost"]);

    let out = run_cmd(&home, &["wallet", "airdrop", "1"]);
    assert_ok(&out, &["wallet", "airdrop", "1"]);

    let keypair_path = home.join(".config").join("spl-forge").join("id.json");
    let kp = read_keypair_file(&keypair_path).expect("failed to read generated keypair");
    let authority = kp.pubkey().to_string();

    let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());
    rpc.request_airdrop(&kp.pubkey(), 2_000_000_000)
        .expect("airdrop request failed on local validator");
    wait_for_airdrop_balance(&rpc, &kp.pubkey());

    let out = run_cmd(
        &home,
        &[
            "create",
            "mint",
            "--mint-authority",
            &authority,
            "--decimals",
            "6",
            "--initial-supply",
            "10",
        ],
    );
    assert_ok(&out, &["create", "mint", "..."]);

    let out = run_cmd(
        &home,
        &[
            "create",
            "token",
            "--name",
            "Smoke Token",
            "--symbol",
            "SMK",
            "--decimals",
            "6",
            "--initial-supply",
            "10",
            "--uri",
            "https://example.com/token.json",
        ],
    );
    assert_fail(&out, &["create", "token", "..."]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("metadata creation is temporarily disabled"));

    let out = run_cmd(
        &home,
        &[
            "create",
            "market",
            "--base-mint",
            &authority,
            "--quote-mint",
            &authority,
        ],
    );
    assert_ok(&out, &["create", "market", "..."]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("not implemented yet"));
}

#[test]
fn wallet_airdrop_rejected_on_mainnet() {
    let home = temp_home();

    let out = run_cmd(&home, &["config", "set", "mainnet"]);
    assert_ok(&out, &["config", "set", "mainnet"]);

    let out = run_cmd(&home, &["wallet", "airdrop", "1"]);
    assert_fail(&out, &["wallet", "airdrop", "1"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Airdrop is only allowed on devnet or localnet"));
}
