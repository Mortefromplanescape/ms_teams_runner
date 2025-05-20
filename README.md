# ms_teams_runner
Запуск MS Teams на движке chrome (без установки приложения и "чтобы не в браузере")

===

Дополнительные оптимизации размера файла:

Установите UPX:

1. Установка UPX (3 способа)
Способ A: Через официальный сайт

Скачайте UPX: https://upx.github.io/

Распакуйте архив в C:\upx

Добавьте в PATH:

Нажмите Win+R → sysdm.cpl → Дополнительно → Переменные среды

В "Системные переменныe" выберите Path → Изменить → Новое

Добавьте путь: C:\upx

Способ B: Через Chocolatey (пакетный менеджер)

Установите Chocolatey (администратор):

powershell
Set-ExecutionPolicy Bypass -Scope Process -Force
iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
Установите UPX:

powershell
choco install upx
Способ C: Через winget (Windows 11)

powershell
winget install UPX.UPX

... и сожмите exe-файл:

upx --best --lzma target/release/chrome-launcher.exe

===

* текущий исходник уже написан с использованием Windows API напрямую. Т.о. конкретно для этого примера оптимизация через upx уже невозможна.