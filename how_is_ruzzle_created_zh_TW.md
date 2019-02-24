# Ruzzle 起源與架構

Want to read the english version of this article? [click here](https://github.com/Rhadow/ruzzle/blob/master/how_is_ruzzle_created_en.md)

2018 年, 我接觸了薩爾達的最新作 *曠野之息*。遊戲裡與自然的互動讓我各種驚豔同時也興起了自己寫遊戲的念頭。說到寫遊戲，大家通常第一反應就是使用市面上的遊戲引擎例如 Unity 等。使用遊戲引擎有許多好處，很多像是物理，渲染，動畫等細節都已經寫好，開發者可以快速製作出遊戲原型並進行遊戲內容開發。但同時這也是缺點，許多這些底層的細節同時也是有趣的地方也被人們遺漏了。我先前有試過用 Unity 開發過一些小遊戲但都無疾而終，這次打算挑戰從零開始打造出一個可以玩的小遊戲，於是 Ruzzle 就誕生了。

## Rust 與 WebAssembly

由於我工作的主要內容是網頁前端，所以在思考遊戲要先開發在哪個平台時，網頁自然就成了首選。 現在在網路上可以看到許多用 WebAssembly 寫的 3D 遊戲樣品，效能好像真的比 Javascript 快不少。在閱讀過幾篇關於 WebAssembly 的文章後就決定把這技術用在 Ruzzle 上了。 大部分的 WebAssembly 都是由其他語言像是 C++, Rust 或 Go 等語言編譯而來，因為我不是可以直接寫組語等級的神人，所以還得挑一個高階語言來編譯成 WebAssembly 才行。又在網路上爬了更多資源後，我決定使用 Rust。主要原因如下：

1. Rust/WebAssembly 的工具鍊目前是相對來說最完整的。在網路上可以找到許多相關教學
2. Rust 似乎蠻適合用來遊戲開發，相關資源：[http://arewegameyet.com/](http://arewegameyet.com/)
3. 既然要學一門新語言，為何不學個 [2018 年被最多人喜愛的語言呢](https://insights.stackoverflow.com/survey/2018/)

這裡順便將用在此專案的工具鍊做個簡介：

### wasm-pack
wasm-pack 目前看起來是最受歡迎的 WebAssembly 工具之一。它的主要功能是使用接下來會提到的工具將 Rust 編譯成可以和 Javascript 溝通的 WebAssembly。同時也可以將編譯好的 WebAssembly 上傳到 npm 供其他人使用。

### wasm-bindgen
wasm-bindgen 是我認為對 Ruzzle 來說最重要的工具。它負責生出讓 Rust 和 Javascript 相互溝通的程式碼。簡單來說，開發者可以在 Javascript 裡呼叫 Rust 寫的函數，也同樣能在 Rust 的程式碼裡呼叫 Javascript 的函數。這全都是靠 wasm-bindgen 的黑魔法才能達成。對他的內部是如何運作有興趣的讀者們可參考[這篇](https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/)。

### js-sys
js-sys 負責把 Javascript 的一些通用函示帶到 Rust 的世界裡。舉一個實際在 Ruzzle 裡用到的例子： Rust 的亂數函式在轉成 WebAssembly 後會無法在瀏覽器上使用，一個解決方法就是使用 js-sys 提供的 `js_sys::Math::random()` 來生成亂數。這相當於呼叫 Javascript 裡的 `Math.random()`。

### web-sys
與 js-sys 類似，不過帶到 Rust 裡的是 Web API 例如 `getElementById` 和 `setTimeout` 之類的。遊戲中的畫面渲染，音效播放等都是透過 web-sys 達成的。使用 web-sys 時要稍微注意一下，如果編譯器報錯說你使用的 Web API 不存在時建議看一下 Cargo.toml 裡的 web-sys 是否有加入你使用的 Web API 功能。基本的內建好像沒有包含所有的 Web API。如果不知道怎麼夾可以參考 Ruzzle 專案的 Cargo.toml。

這裡推薦大家[這份 rust-wasm 教學](https://rustwasm.github.io/book/introduction.html)，以上提及的工具跟著教學走都會自動加入到你的專案中。如果以上有錯誤的部分也請大家指正，我也還正在熟悉這些工具中。

### 如何建立 Ruzzle 開發環境

在此段最後，我想快速講一下如何在本地建立 Ruzzle 的開發環境，以下命令只需執行一次即可：

1. cd ruzzle
2. wasm-pack build
3. npm init wasm-app www
4. cd www
5. npm install
6. cd ruzzle/pkg
7. npm link
8. cd ruzzle/www
9. npm link ruzzle
10. npm run start

我知道你會抱怨怎麼這麼麻煩，詳細的原因都有在上面提到的 rust-wasm 教學中有說明。如果你想要懶人包的話，請看這：基本上除了 `ruzzle/pkg` 與 `ruzzle/www` 以外的資料夾都是與 Rust 相關的。wasm-pack 會把這裡的 Rust 程式碼編譯成 WebAssembly 並放到 `ruzzle/pkg` 中。`ruzzle/www` 則是放與前端相關的程式碼，這裡透過 `npm link` 在 `ruzzle/www` 中建立一個編譯好的 WebAssembly 捷徑供它使用。最後 webpack dev server 把整個 `ruzzle/www` 資料夾裡的內容跑起來。

在開發時，每當有 Rust 程式碼改動時都需要手動跑一次 `wasm-pack build` 來編譯最新的程式碼後才能看到結果。webpack-dev-server 則不用重啟，它會自動發現有更動並反映在網頁上。目前的開發流程並不是這麼有效率，原因主要是現在 WebAssembly 與網頁開發都還在起步中，工具也都不是很成熟，相信這些問題都會在未來得到改善。

## Ruzzle 遊戲內容

起初，我本來是想做出類似任天堂發表的荒野之息 2D 原型，[影片在這](https://www.youtube.com/watch?v=ruNLBHDS3yM)。但後來發現已經有人做出來了，再說開發這等級的遊戲對像我這樣的新手來說也太難了，於是決定先做個簡單一點的版本。在搜尋 Rust 遊戲開發時有看到一個不錯的[解謎遊戲](http://www.luduminis.com/pascal/)，我想說把荒野之息裡的元素帶入或許蠻好玩的，就決定往這方向下去做了。最後對遊戲的 MVP 定義是 2D 解謎遊戲，能互動的物件有砲台，岩石與火燄。喔對了，如果你好奇為什麼這遊戲叫 Ruzzle, 其實就只是把 Rust 和 Puzzle 混在一起做撒尿牛丸而已。

## 遊戲架構

這段應該是本文中跟遊戲最有相關的部分，主要會講述遊戲裡的各個模組與他們負責的內容。在這裡先說一下，這是我第一次從頭開發遊戲，架構一定不是完美的，要鞭請小力點。事實上，在我開發結束後，我發現了[這篇文章](https://kyren.github.io/2018/09/14/rustconf-talk.html)，看完後真的是有夠後悔為什麼沒早點讀到。主要是在講用 Rust 開發遊戲的架構，內容各種戳中此專案的痛點。如果你也想用 Rust 開發遊戲，我強烈建議讀完再來寫扣。

好啦，預防針打完了, 我們開始吧：

### Javascript

Javascript 的進入點是 `ruzzle/www/index.js`。由於主要的遊戲邏輯都由 Rust 來處理，Javascript 的主要任務就只有負責載入圖片音效，綁定使用者輸入事件與渲染而已。在這裡會從 WebAssembly 引入一個叫 `WebClient` 的物件，它其實就是遊戲本身並額外提供一些方便的介面例如 `update` 和 `render` 供 Javascript 使用。

### Rust

在 Rust 這邊，進入點則是 `ruzzle/src/lib.rs`，主要細分為以下模組：

- client
- renderer
- audio
- controller
- utils
- game

### Client

Client 主要對應的是遊戲執行的平台。每個平台處理渲染，播放音效，綁定事件的方法都不同。例如 Web 就是使用 canvas 與 HTMLAudioElement 來做畫面渲染與音效。為了能夠讓 Ruzzle 在未來能夠輕易轉換平台，我把這部分邏輯抽離遊戲本體並提供統一的介面供遊戲使用。這樣那些不統一的部分就讓以下會提到的模組來處理。Client 裡也包含了遊戲本身與其他一些資料。

### Renderer

Renderer 主要負責渲染畫面。Renderer 的統一介面定義在 `ruzzle/src/renderer/mod.rs`。如以上提到的，對遊戲本身來說，當有需要顯示物品時，它只需要使用 Renderer 的 `draw_objects` 方法。平台如何實作把物品畫到畫面上的邏輯它是不需要知道的。

### Audio

同理於 Renderer，負責音效。統一介面定義在 `ruzzle/src/audio/mod.rs`。

### Controller

Controller 負責將玩家的輸入對應到遊戲中。原本 Controller 是沒有被獨立出來成為一個模組的，本來的做法是直接將遊戲邏輯綁定到輸入事件上，例如：每當有 keydown 事件發生直接呼叫 `handle_player_movement`。但這種作法在操作上會有極大的延遲。我不確定這是否是 web 才有這問題，上了 stackoverflow 後才發現大家都建議不要直接呼叫遊戲邏輯，而是將輸入先對應到 map 上，遊戲再讀取 map 來做運算。用 Javascript 寫的話，Map 大概會長這樣：

```
{
    isDownPressed: true,
    isUpPressed: false,
    ...
}
```

### Utils

這裡負責放一些公用函數，例如用在移動動畫的 `coordinate_lerp` 和處理碰撞的 `check_collision`。順帶一提，Ruzzle 碰撞機制是用矩形重疊面積來處理。Leetcode 上有[這題](https://leetcode.com/problems/rectangle-area/)，之前 leetcode 總算沒白刷有實用到了啊啊啊！

### Game

接下來要說說遊戲本體了，由於太大的關係，它被進一步的分解成以下模組：

- scenes
- assets
- constants
- status_manager
- terrains
- tile
- character
- objects
- level
- world

### Scenes

在 Ruzzle 裡有三個主要畫面：起始畫面，選關畫面與遊戲畫面。我一樣為畫面定義了統一介面讓它們可以有各自對應不同事件的處理方法。接著來說說如何做畫面切換。每個畫面都有一個可為空的 `next_scene` 屬性。當需要切換畫面時，當前畫面會根據它 `update` 方法裡的邏輯設定 `next_scene`。接著，Client 的 `update` 會在每一幀檢查當前畫面的 `next_scene` 是否為空，如果不是空的話就渲染下一個畫面。我自己認為這做法不是很好，目前想到的重構方向是使用 [Observer Pattern](http://www.gameprogrammingpatterns.com/observer.html)。

### Assets

Asset 是用來儲存遊戲中物件顯示相關的資料。在 Ruzzle 裡有四張貼圖，當遊戲想畫一棵樹時，我們需要告訴他用哪張貼圖，樹在貼圖中的位置還有樹在貼圖中的大小。當物件狀態改變例如樹燒起來時，就需要更新這裡的資料讓它對應到燒樹的貼圖才能夠在畫面上顯示出來。抽離 Asset 成一個模組的好處是當有多個物件邏輯是相同但畫面顯示卻不同時，就能夠把邏輯部分重用只需要修改 Asset 即可。

### Constants

遊戲常數，沒什麼特別的。

### Status Manager

Status manager 主要控制物件的狀態。在遊戲中的大小，位置，方向等等。遊戲中的所以物件 (人物，物品，地形) 都有這個屬性。這也同時是個設計缺失，例如地形根本不需要知道它現在在走路或是快死了，但這些資料卻儲存在地形底下。重構的方向是捨棄類 OO 的寫法並引入 ECS (Entity Component System)。Rust 沒有繼承的語法所以我寫類 OO。

順帶一提，讓角色與物品在格子間流暢移動的邏輯也寫在這。由於這遊戲地圖主要是格子組成，遊戲物件都用位置(行，列)來表示所在地。但這樣的做法會使物件在移動中瞬移到下一格，不會有物件在格子中間移動的畫面。這裡的解法是多加入一個座標(對應於 canvas 的 x, y)屬性，這樣就可以讓物件格子中飄移。Status Manager 裡也提供方法讓座標與位置互相轉換。

### Terrains

目前在遊戲中只有兩種地形：平地 (`Land`) 與 落穴 (`Hole`) (遊戲王臉)。地形也有提供統一的介面將各種地形的處理邏輯抽象化。

### Tile

遊戲的地圖其實就是一個塞滿一堆格子的陣列，在遊戲裡叫 `tile_map`。目前每個格子裡只有裝該位置的地形而已。雖然格子現在只是一個負責包地形的，但未來預計會在每個格子中加入特殊效果例如發光之類的，到時就知道它厲害了。

### Character

負責放角色程式的地方，一樣有提供統一接口。

### Objects

物品是你在地圖上看到除了人物之外的所有東西。物品有一個負責控制他們互動行為的額外屬性叫 `attribute_manager`。例如用來看物件現在多熱的溫度，物件可不可燃，可不可以被推動之類的屬性就會被放在 `attribute_manager` 裡。它其實與 `status_manager` 相當類似，差別就在於只有物品有的才放在 `attribute_manager` 裡。如果未來人物也需要溫度的話，那就要把溫度移到 `status_manager` 了。個人認爲這設計也是不太好，用 ECS 應該能解決掉這問題。

### Level

遊戲中的關卡與關卡編輯器都放在這模組裡。每個關卡是由三個元素組成：地形，物品與玩家初始位置。地形與物品在關卡編輯器裡都是字串陣列。每個字串都有對應的遊戲物件，例如 'T' 對應到樹 (Tree)。完整的對應文件可在 `ruzzle/src/game/level/level_manager.rs` 找到。玩家初始位置是用行與列組成的 Tuple 來組成，復活點(物品)會自動放置在玩家初始位置。

如果你好奇復活點是做什麼的，答案是避免玩家把石頭，火堆推到復活點。因為玩家 GG 後會被送回復活點，要是那裡有石頭的話我不就 GG 了嗎，當然要擋一下囉。

### World

遊戲世界也就是一個關卡是由上面所有模組組成。世界裡儲存著地圖，物品資訊，角色資訊與關卡相關資訊。世界模組裡也提供許多拿取/更改遊戲物件的方法讓各個模組使用。

## 遊戲素材

呼～終於把困難的看完了。動完左腦現在該來動動右腦了。說到遊戲素材像是角色貼圖，遊戲音樂等等對我這碼農來說根本很遙遠。不要說音樂了，我從高中畢業後大概就只有畫過流程圖和 "[樹](https://en.wikipedia.org/wiki/Tree_(data_structure))" 了。所以剛開始的想法是到 [opengameart.org](https://opengameart.org/) 等網站拿免費素材來用。但是說真的，我做這些玩具專案的主要意圖就是學點新東西，所以我決定自己的遊戲貼圖自己畫！你可能會問我那音樂勒？其實我也有想過自己來寫寫看，也在 youtube 上找到很酷的[頻道](https://www.youtube.com/channel/UCeZLO2VgbZHeDcongKzzfOw)，但實際開始畫圖後才知道隔行如隔山。做這遊戲的一半時間都在畫圖，寫音樂就下次吧，我還是偏好寫扣。

接下來分享一下畫圖心得。我畫的是我本來以為很簡單的像素畫 (Pixel Art)。使用的工具是網路上大多數人推薦的 [aseprite](https://www.aseprite.org/)。這東西真的是畫下去才知道難。因為像素少，很多東西需要經過計算看起來才會自然，有些細節甚至要捨棄。還有其他專業的部分樣式各種陰影，視角等要注意。總之，要畫出一張能看的貼圖真的需要大量的練習與對我們周遭世界的觀察。畫完之後真的對畫家各種敬佩。

如果你也有興趣畫像素畫的話，可以參考以下資源：

- [https://medium.com/@davidbyttow/a-quick-and-dirty-guide-to-creating-pixel-art-d3d43d4bf421](https://medium.com/@davidbyttow/a-quick-and-dirty-guide-to-creating-pixel-art-d3d43d4bf421)
- [http://blog.studiominiboss.com/pixelart](http://blog.studiominiboss.com/pixelart)
- [https://medium.com/pixel-grimoire](https://medium.com/pixel-grimoire)

## 總結

用 Rust 開發遊戲是個讓人又愛又恨的經驗。它強大的編譯器讓你在做修改時不用提心吊膽，有什麼錯直接抓出來不手軟。但另一方面，每當我看到編譯器吐給我
`Cannot borrow as mutable because it is also borrowed as immutable` 或 `cannot infer an appropriate lifetime for lifetime parameter` 我就想罵Ｘ。總體來說，我認為寫 Rust 可以讓你用不同視角來看待問題。Rust 編譯器抓出來的錯誤通常都是在其他語言中造成很難解的 bug 的原因。當你剛開始寫 Rust 時，你可能需要兩到三小時把一個錯誤訊息給消化掉。但一但熟悉之後，你可以越來越快的找到問題的原因甚至預先就避免掉。總體來說我是推薦 Rust 的。

再來説説遊戲部份。我其實一直都知道寫遊戲並不簡單，但在實際寫之後才發現寫好遊戲真的很難。我列了個人認為一個好遊戲必須要的條件：

1. 好的遊戲架構方便擴充改寫
2. 好的遊戲素材 (貼圖，音樂)
3. 好的關卡設計讓玩家深陷其中

我列出的有線，相信大家也有自己的看法。現階段的 Ruzzle 以上三點都沒有達成，也還有一堆功能沒完成。但這 MVP 嘛，未來還是有計劃加入儲存進度，更多互動元素，改善操控等等功能，但什麼時候完成不知道就是。

以一個玩具專案來說，我的確學到了不少。Rust, webassembly, 遊戲開發, 像素圖對我來說都是之前沒碰過的東西。做出來的結果也不算差，至少還可以玩。
最後，感謝你耐心讀完這篇，如果你也想參與 Ruzzle 的開發，歡迎發ＰＲ！
