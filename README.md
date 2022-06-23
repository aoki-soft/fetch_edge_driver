# ネットからedgedriverをダウンロードして動くようにする。

[edgedriverのバージョン取得コマンド](https://qiita.com/p1nk5p1der/items/fe7d076b061bd8396af9)  

```PowerShell
(get-item ($env:SystemDrive + "\Program Files (x86)\Microsoft\Edge\Application\msedge.exe")).VersionInfo.FileVersion
```


https://msedgedriver.azureedge.net/102.0.1249.0/edgedriver_win32.zip

https://msedgedriver.azureedge.net/102.0.1249.0/edgedriver_win64.zip
https://msedgedriver.azureedge.net/102.0.1245.44/edgedriver_win64.zip