# sensui code
とりあえず、main はとりあえず攻撃とか防御(?)とか、最低限のコードしか置いていません。戦略とか変えるならブランチを切っていく方針で。

## Requirements
+ Rust(1.48.0)

## Install
### 1. Rust のインストール(ない人だけ)
```console
$ sudo apt install curl
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
.
.
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```
ここまでできたら `1` と入力して、次のコマンドで環境変数をセットしたら完了
```console
$ source $HOME/.cargo/env
```

### 2. sensui-code の clone
勉強会参加した人は公開鍵を置いているはずなので SSH でやりましょう。
```console
$ cd {clone 先のディレクトリ}
$ git clone git@github.com:earlgray283/sensui-code.git
$ cd sensui-code
```

## Usage
```console
$ cargo run
```