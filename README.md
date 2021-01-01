# sensui code
ぼくがかんがえたさいきょーの潜水艦ゲームコンピュータです。

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

## Update
度々修正しているので実行する前に remote の変更を local に反映させてください。
```console
$ git pull origin main
```

## Execute
```console
$ cargo run
```

## Release Build
最高のパフォーマンスを、あなたに
```console
$ cargo build --release
```

## Explain
### 1. テーブル画面
```console
+-----------+
| . . # . . |
| . . . . . |
| # . . . # |
| . . . . . |
| . . # . . |
+-----------+
+------------------------------------------------------------------+
|           -1           -1           -1           -1           -1 |
|           -1           -1           -1           -1           -1 |
|           -1           -1           -1           -1           -1 |
|           -1           -1           -1           -1           -1 |
|           -1           -1           -1           -1           -1 |
+------------------------------------------------------------------+
```
上に自分の現在の潜水艦の配置、下に確率テーブルが表示されます。

### 2. ログとプロンプト
#### 2-1. 攻撃
```console
attack to ({x}, {y})!
input attack result: (hit / rage / dead / none) > 
結果を入力
```
結果を入力した後にテーブルが更新されて表示されます。

#### 2-2. 防御(相手の行動を受け取る)
```console
input query: (1 x y / 2 d n) >
クエリを入力
```

`1 x y` は、相手が `(x, y)` に攻撃することを示します。  
`2 d n` は、相手が `d` 方向に `n` マス移動することを示します。 