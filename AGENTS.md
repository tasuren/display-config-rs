# AGENTS.md

このライブラリは、ディスプレイの増減や解像度の変更を追跡するたのクレートである。
プロジェクト名はdisplay-config-rsで、クレートの名前はdisplay-configである。

## 目的

このクレートは、RustのGUIアプリケーションの開発において、各ディスプレイにウィンドウを配置しておきたい、
というケースに対応するために開発が始まった。
例えば、画面上にちょっとしたペン書きをするOn-screen anotationアプリでは、ペン書きした絵を表示する
ウィンドウを各ディスプレイに配置する必要がある。そして、ディスプレイが増えたら増えた分そのウィンドウを増やす必要がある。

このような、ディスプレイ構成の変更に関心があるGUIアプリ開発を支援するためのクレートの開発が、このプロジェクトの目的である。

## 機能

- `get_display`: 指定されたIDのディスプレイ情報を取得する。
- `get_displays`: ディスプレイ情報を全て取得する。
- `Display`: ディスプレイ情報を含む構造体。
- `DisplayObserver`: ディスプレイ構成を監視する機能を実装した構造体。
- `MayBeDisplayAvailable`: 構成変更イベントをラップする。`Removed`イベントのようなディスプレイ無効時を考慮する。
- `Event`: ディスプレイ構成変更イベントを表現する列挙型。

## アーキテクチャ

プラットフォーム別の実装は、`src/macos.rs`にmacOS、`src/windows.rs`にWindows向けの実装としてあり、
モジュール毎に分けられている。Linuxはいつか対応する予定であるが、現在は考慮しない。  
基本的にプラットフォーム固有の実装は前述のmacosモジュールとwindowsモジュールで行う。

`lib.rs`では、プラットフォーム固有の実装を
`PlatformDisplayObserver`や`PlatformDisplayId`というようなPlatformをプリフィックスとして
インポートし、それをラップした`DisplayObserver`や`DisplayId`を提供する。

なお、ユーザー側がプラットフォーム固有の実装を直接使う場合は、このPlatformをプリフィックスとするもの
ではなく、macosモジュール、windowsモジュールにある`MacOSDisplay`や`WindowsDisplay`といった、
OS名をプレフィックスとするものを直接使う。これは誤ってクロスプラットフォームではない
`PlatformDisplayObserver`などを使わないようにするためである。`MacOSDisplay`を使った場合、
Windowsではエラーになる。

### プラットフォーム固有の設計

#### Windows

WindowsではディスプレイのIDとしてデバイスパスを用いる。当初は`HMONITOR`を使う予定であったが、
設定が変わる毎に無効になる可能性があるため、デバイスパスを使うことになった。

イベントの追跡には、見えないウィンドウを作り、そのイベントとして`WM_DISPLAYCHANGE`を受け取る。
ただ、これにはどのディスプレイの何の設定が変更されたのか不明なので、構造体`EventTracker`にて
設定変更の追跡を行う。（この構造体は外部に公開しない。）

#### macOS

macOSではIDとして`CGDirectDisplayID`を用いる。`MacOSDisplayId`として`CGDirectDisplayID`の
型エイリアスを提供する。`MacOSDisplay`はこのIDを所持し、諸々の情報にアクセスする。

イベントの追跡には、`CGDisplayRegisterReconfigurationCallback`を用いる。  
ただ、これによるイベント配信は、解像度の変更を追跡できないため、解像度の情報はキャッシュし自分で追跡を行う。
この実装は構造体`EventTracker`にある。（この構造体は外部に公開しない。）
