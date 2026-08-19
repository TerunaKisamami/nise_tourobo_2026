# Mini‑a 完全移植ガイド 🚀

このリポジトリの **`main` ブランチ** に含まれるコードを、ロボット搭載用PC（mini‑a）へそのまま移植できるよう、必要なコマンド・手順をすべてまとめました。

---

## 📋 前提条件
- mini‑a は Ubuntu 22.04 (ROS 2 **Humble**) がインストールされた状態で利用します。
- ネットワークが利用でき、GitHub へアクセス可能であること。
- 権限が必要な操作は `sudo` が使えるユーザーで実行してください。

---

## 1️⃣ ROS 2 本体と開発ツールのインストール
```bash
# ROS 2 Humble と基本開発ツールをインストール
sudo apt update && sudo apt install -y \
    ros-humble-desktop \
    python3-colcon-common-extensions \
    python3-rosdep \
    python3-rosinstall-generator \
    git curl

# ROS の環境をロード（この行は .bashrc に追記しても OK）
source /opt/ros/humble/setup.bash
```
> **Tip**: `source /opt/ros/humble/setup.bash` を毎回書くのが面倒な場合は `echo 'source /opt/ros/humble/setup.bash' >> ~/.bashrc` として永続化できます。

---

## 2️⃣ rosdep の初期化（まだやっていない場合）
```bash
# rosdep データベースの初期化（一度だけ実行）
sudo rosdep init
rosdep update
```
---

## 3️⃣ ワークスペース作成 & コード取得
```bash
# ワークスペース作成（~/robobobo が推奨）
mkdir -p ~/robobobo/nise_tourobo_2026/src
cd ~/robobobo/nise_tourobo_2026/src

# 本体リポジトリをクローン（現在のリポジトリ）
git clone https://github.com/TerunaKisamami/tourobo_2026_auto.git .

# 依存であるカスタムインタフェースやメッセージ定義も取得
# （ros2_packages には dyna_interfaces, tourobo_2026_interfaces などが入っている）
git clone https://github.com/TerunaKisamami/ros2_packages.git
```
---

## 4️⃣ 依存パッケージの自動解決 & ビルド
```bash
cd ~/robobobo/nise_tourobo_2026

# rosdep が apt パッケージへ自動で変換し、足りない依存をインストール
rosdep install -i --from-path src --rosdistro humble -y

# ビルド（--symlink-install で開発時の再ビルドが速くなる）
colcon build --symlink-install

# ビルドした環境を現在のシェルに反映
source install/setup.bash
```
> **Note**: 途中で `apt` パッケージが見つからない場合は、エラーメッセージに表示されたパッケージ名で `sudo apt install <pkg>` を手動で実行してください。

---

## 5️⃣ 環境永続化（ターミナル起動時に自動ロード）
```bash
# .bashrc に追記しておくと、毎回 source が不要になります
echo 'source ~/robobobo/nise_tourobo_2026/install/setup.bash' >> ~/.bashrc
```
---

## 6️⃣ シリアル/USB デバイス権限設定
Dynamixel や CAN‑USB アダプタは `/dev/ttyUSB*` に接続されることが多いので、一般ユーザーがアクセスできるようにします。
```bash
# dialout グループに自分を追加（再ログインまたは再起動が必要）
sudo usermod -a -G dialout $USER

# udev でデバイス名を固定化（例: Dynamixel が idVendor=2a00, idProduct=0015 のとき）
cat <<EOF | sudo tee /etc/udev/rules.d/99-dynamixel.rules
SUBSYSTEM=="tty", ATTRS{idVendor}=="2a00", ATTRS{idProduct}=="0015", SYMLINK+="dynamixel0", MODE="0666"
EOF
sudo udevadm control --reload-rules && sudo udevadm trigger
```
---

## 7️⃣ ハードウェア（Joy、カメラ等）の確認
### Joy コントローラ
```bash
ros2 run joy joy_node &
# 別ターミナルで
ros2 topic echo /joy
```
### USB カメラ（例: `usb_cam`）
```bash
ros2 run usb_cam usb_cam_node &
ros2 topic echo /image_raw
```
これらが正常にトピックを流していれば、mini‑a 側でもハードウェアは認識されています。
---

## 8️⃣ 起動例（全ノードを一括起動）
```bash
# ROS 2 デーモンをリロード（環境変化があったときは）
ros2 daemon stop && ros2 daemon start

# 例: joy_mechanism_client を起動
ros2 launch tourobo_2026_mechanisms joy_mechanism_client.launch.py
```
> **Tip**: 各ノードは `ros2 run <package> <executable>` でも個別起動可能です。

---

## 📂 生成されたファイル一覧（リポジトリ内）
```
MIGRATION_GUIDE.md   ← 本ガイド（今回追加したファイル）
src/
  ├─ nise_tourobo_2026/           # 本体コード
  └─ ros2_packages/               # カスタムメッセージ・インタフェース
install/                           # colcon ビルド結果（生成物）
```
---

## ✅ 完了チェックリスト
- [ ] ROS 2 Humble がインストール済み
- [ ] `rosdep` 初期化済み
- [ ] `~/robobobo/nise_tourobo_2026` にリポジトリがクローン済み
- [ ] 依存パッケージを `rosdep install` で解決
- [ ] `colcon build` がエラーなしで完了
- [ ] `source install/setup.bash` が実行できる
- [ ] USB デバイス権限が設定済み (`dialout` グループ、udev ルール）
- [ ] Joy / カメラ が `/joy` や `/image_raw` でトピック配信確認
- [ ] `ros2 launch tourobo_2026_mechanisms joy_mechanism_client.launch.py` が起動できる

---

## 📦 GitHub への反映手順（自動化済み）
このガイドは **`MIGRATION_GUIDE.md`** としてリポジトリのルートに追加し、次のコミットでプッシュしています。
```bash
git add MIGRATION_GUIDE.md
git commit -m "Add full mini-a migration guide"
git push origin main
```

以上で、**mini‑a への移植に必要なすべてのコマンドと手順**が完了です。実機でテストしてみて、エラーや不明点が出たら遠慮なく教えてください！
