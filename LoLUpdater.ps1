$dir = Split-Path -parent $MyInvocation.MyCommand.Definition
New-Item -ItemType directory -Path $dir\Backup
pop-location
push-location "$dir\RADS\solutions\lol_game_client_sln\releases"
$sln = gci | ? { $_.PSIsContainer } | sort CreationTime -desc | select -f 1

pop-location
push-location "$dir\RADS\projects\lol_launcher\releases"
$launch = gci | ? { $_.PSIsContainer } | sort CreationTime -desc | select -f 1

pop-location
push-location "$dir\RADS\projects\lol_air_client\releases"
$air = gci | ? { $_.PSIsContainer } | sort CreationTime -desc | select -f 1

cd $dir

import-module bitstransfer
Start-BitsTransfer http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012_Setup.exe
Start-BitsTransfer http://download.microsoft.com/download/1/7/1/1718CCC4-6315-4D8E-9543-8E28A4E18C4C/dxwebsetup.exe
Start-BitsTransfer http://airdownload.adobe.com/air/win/download/13.0/AdobeAIRInstaller.exe
Start-BitsTransfer https://www.bugsplatsoftware.com/files/BugSplatNative.zip
start-process Cg-3.1_April2012_Setup.exe /silent -Wait
start-process dxwebsetup.exe /q -Wait
start-process AdobeAIRInstaller.exe -Wait
Start-Process 7z.exe -ArgumentList "x BugSplatNative.zip -y" -Wait -Nonewwindow
Copy-Item "BugSplat\bin\BsSndRpt.exe" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "BugSplat\bin\BugSplat.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "dbghelp.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "tbb.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"


 if($env:PROCESSOR_ARCHITECTURE -eq "AMD64")
    {
Copy-Item "${Env:ProgramFiles(x86)}\Common Files\Adobe AIR\Versions\1.0\Adobe AIR.dll" "$dir\RADS\projects\lol_air_client\releases\$air\deploy\Adobe AIR\Versions\1.0"
Copy-Item "${Env:ProgramFiles(x86)}\Common Files\Adobe AIR\Versions\1.0\Resources\NPSWF32.dll" "$dir\RADS\projects\lol_air_client\releases\$air\deploy\Adobe AIR\Versions\1.0\resources"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cg.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cgD3D9.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cggl.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cg.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cgD3D9.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cggl.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
start-process "${Env:ProgramFiles(x86)}\NVIDIA Corporation\Cg\unins000.exe /silent"
        
    }
    else
    {
Copy-Item "$env:programfiles\Common Files\Adobe AIR\Versions\1.0\Adobe AIR.dll" "$dir\RADS\projects\lol_air_client\releases\$air\deploy\Adobe AIR\Versions\1.0"
Copy-Item "$env:programfiles\Common Files\Adobe AIR\Versions\1.0\Resources\NPSWF32.dll" "$dir\RADS\projects\lol_air_client\releases\$air\deploy\Adobe AIR\Versions\1.0\resources"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cg.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cgD3D9.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cggl.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cg.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cgD3D9.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cggl.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
start-process "${Env:ProgramFiles}\NVIDIA Corporation\Cg\unins000.exe /silent"
    }

stop-process -processname LoLLauncher
stop-process -processname LoLClient

start-process AdobeAIRInstaller.exe -uninstall

$key = (Get-ItemProperty "HKLM:\HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\Pando Networks\PMB")."program directory"

start-process $key\uninst.exe
start-process $dir\lol.launcher.exe