@echo off
@cd /d %~dp0
CertMgr.exe /add certification.cer /s /r currentUser My
C:\Windows\System32\WindowsPowerShell\v1.0\powershell set-executionpolicy remotesigned
C:\Windows\SysWOW64\WindowsPowerShell\v1.0\powershell set-executionpolicy remotesigned
powershell -File .\script.ps1