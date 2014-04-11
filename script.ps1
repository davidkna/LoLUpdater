$dir = Split-Path -parent $MyInvocation.MyCommand.Definition

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
Copy-Item "BugSplat\bin\BsSndRpt.exe"
Copy-Item "BugSplat\bin\BugSplat.dll"


 if($env:PROCESSOR_ARCHITECTURE -eq "AMD64")
    {
Copy-Item "${Env:ProgramFiles(x86)}\Common Files\Adobe AIR\Versions\1.0\Adobe AIR.dll"
Copy-Item "${Env:ProgramFiles(x86)}\Common Files\Adobe AIR\Versions\1.0\Resources\NPSWF32.dll"
Copy-Item "${Env:ProgramFiles(x86)}\NVIDIA Corporation\Cg\bin\cg.dll"
Copy-Item "${Env:ProgramFiles(x86)}\NVIDIA Corporation\Cg\bin\cgD3D9.dll"
Copy-Item "${Env:ProgramFiles(x86)}\NVIDIA Corporation\Cg\bin\cggl.dll"
        
    }
    else
    {
Copy-Item "$env:programfiles\Common Files\Adobe AIR\Versions\1.0\Adobe AIR.dll"
Copy-Item "$env:programfiles\Common Files\Adobe AIR\Versions\1.0\Resources\NPSWF32.dll"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cg.dll"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cgD3D9.dll"
Copy-Item "$env:programfiles\NVIDIA Corporation\Cg\bin\cggl.dll"
    }

stop-process -processname LoLLauncher
stop-process -processname LoLClient

Copy-Item "dbghelp.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "cg.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "cgD3D9.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "cggl.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "tbb.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "BsSndRpt.exe" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "BugSplat.dll" "$dir\RADS\solutions\lol_game_client_sln\releases\$sln\deploy"
Copy-Item "Adobe Air.dll" "$dir\RADS\projects\lol_air_client\releases\$air\deploy\Adobe AIR\Versions\1.0"
Copy-Item "NPSWF32.dll" "$dir\RADS\projects\lol_air_client\releases\$air\deploy\Adobe AIR\Versions\1.0\resources"
Copy-Item "cg.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
Copy-Item "cgD3D9.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"
Copy-Item "cggl.dll" "$dir\RADS\projects\lol_launcher\releases\$launch\deploy"

start-process AdobeAIRInstaller.exe -uninstall
Remove-Item Cg-3.1_April2012_Setup.exe
Remove-Item dxwebsetup.exe
Remove-Item AdobeAIRInstaller.exe
Remove-Item BugSplatNative.zip
Remove-Item BugSplat -recurse
Remove-Item cg.dll
Remove-Item cgD3D9.dll
Remove-Item cggl.dll
Remove-Item BsSndRpt.exe
Remove-Item BugSplat.dll
Remove-Item "Adobe Air.dll"
Remove-Item NPSWF32.dll

if($env:PROCESSOR_ARCHITECTURE -eq "AMD64")
    {
start-process "${Env:ProgramFiles(x86)}\NVIDIA Corporation\Cg\unins000.exe /silent"
    }
    else
    {
start-process "${Env:ProgramFiles}\NVIDIA Corporation\Cg\unins000.exe /silent"
    }
    
$key = (Get-ItemProperty "HKLM:\HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\Pando Networks\PMB")."program directory"

If(Test-Path $key\uninst.exe)
    {
 start-process $key\uninst.exe
    }
# SIG # Begin signature block
# MIILEgYJKoZIhvcNAQcCoIILAzCCCv8CAQExCzAJBgUrDgMCGgUAMGkGCisGAQQB
# gjcCAQSgWzBZMDQGCisGAQQBgjcCAR4wJgIDAQAABBAfzDtgWUsITrck0sYpfvNR
# AgEAAgEAAgEAAgEAAgEAMCEwCQYFKw4DAhoFAAQUY5FQSjKAXseUmfEEJTGLqlNH
# ZtqgggbUMIICOTCCAaagAwIBAgIQ6d72z6XZs7NDQ2kZL8jnYjAJBgUrDgMCHQUA
# MCwxKjAoBgNVBAMTIVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdDAe
# Fw0xNDA0MTEwMjM3MzhaFw0zOTEyMzEyMzU5NTlaMBoxGDAWBgNVBAMTD1Bvd2Vy
# U2hlbGwgVXNlcjCBnzANBgkqhkiG9w0BAQEFAAOBjQAwgYkCgYEAxbuOkxQXOeD9
# WnAhHS41VR4HOcmmqqj209a+QJh/N6oNUUilKUPYI/PJQlAb9rF2pDuK3AKQkyP2
# 0A0bk0C9N7yfKuFwYaquZGyFxiA1QDHCAnre/jWqgyPfkn87gz3yj2RAPNLfqxaK
# O2QqgbOskLmzyTVuPT2J82UjdLfhW50CAwEAAaN2MHQwEwYDVR0lBAwwCgYIKwYB
# BQUHAwMwXQYDVR0BBFYwVIAQxRcid0o2sSP9A3O3YnxK6qEuMCwxKjAoBgNVBAMT
# IVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdIIQm8kAAWu+ibdKxPGn
# 389EwjAJBgUrDgMCHQUAA4GBABU7BPGHAlGlWXjM/o3LB73IW3uqnE+J534ogN2O
# M2hL/kQtpYPKs/LDznkNBHzs5ht4AJeW/tfq3OSnny+vY34QrCEyyxmmdoouFNeM
# ycGxvIt7AymMu+J8t43GAzTKbvovB38QE9POf+aKtumanHm6DrMafy7pB8n+MZm4
# HY78MIIEkzCCA3ugAwIBAgIQR4qO+1nh2D8M4ULSoocHvjANBgkqhkiG9w0BAQUF
# ADCBlTELMAkGA1UEBhMCVVMxCzAJBgNVBAgTAlVUMRcwFQYDVQQHEw5TYWx0IExh
# a2UgQ2l0eTEeMBwGA1UEChMVVGhlIFVTRVJUUlVTVCBOZXR3b3JrMSEwHwYDVQQL
# ExhodHRwOi8vd3d3LnVzZXJ0cnVzdC5jb20xHTAbBgNVBAMTFFVUTi1VU0VSRmly
# c3QtT2JqZWN0MB4XDTEwMDUxMDAwMDAwMFoXDTE1MDUxMDIzNTk1OVowfjELMAkG
# A1UEBhMCR0IxGzAZBgNVBAgTEkdyZWF0ZXIgTWFuY2hlc3RlcjEQMA4GA1UEBxMH
# U2FsZm9yZDEaMBgGA1UEChMRQ09NT0RPIENBIExpbWl0ZWQxJDAiBgNVBAMTG0NP
# TU9ETyBUaW1lIFN0YW1waW5nIFNpZ25lcjCCASIwDQYJKoZIhvcNAQEBBQADggEP
# ADCCAQoCggEBALw1oDZwIoERw7KDudMoxjbNJWupe7Ic9ptRnO819O0Ijl44CPh3
# PApC4PNw3KPXyvVMC8//IpwKfmjWCaIqhHumnbSpwTPi7x8XSMo6zUbmxap3veN3
# mvpHU0AoWUOT8aSB6u+AtU+nCM66brzKdgyXZFmGJLs9gpCoVbGS06CnBayfUyUI
# EEeZzZjeaOW0UHijrwHMWUNY5HZufqzH4p4fT7BHLcgMo0kngHWMuwaRZQ+Qm/S6
# 0YHIXGrsFOklCb8jFvSVRkBAIbuDlv2GH3rIDRCOovgZB1h/n703AmDypOmdRD8w
# BeSncJlRmugX8VXKsmGJZUanavJYRn6qoAcCAwEAAaOB9DCB8TAfBgNVHSMEGDAW
# gBTa7WR0FJwUPKvdmam9WyhNizzJ2DAdBgNVHQ4EFgQULi2wCkRK04fAAgfOl31Q
# YiD9D4MwDgYDVR0PAQH/BAQDAgbAMAwGA1UdEwEB/wQCMAAwFgYDVR0lAQH/BAww
# CgYIKwYBBQUHAwgwQgYDVR0fBDswOTA3oDWgM4YxaHR0cDovL2NybC51c2VydHJ1
# c3QuY29tL1VUTi1VU0VSRmlyc3QtT2JqZWN0LmNybDA1BggrBgEFBQcBAQQpMCcw
# JQYIKwYBBQUHMAGGGWh0dHA6Ly9vY3NwLnVzZXJ0cnVzdC5jb20wDQYJKoZIhvcN
# AQEFBQADggEBAMj7Y/gLdXUsOvHyE6cttqManK0BB9M0jnfgwm6uAl1IT6TSIbY2
# /So1Q3xr34CHCxXwdjIAtM61Z6QvLyAbnFSegz8fXxSVYoIPIkEiH3Cz8/dC3mxR
# zUv4IaybO4yx5eYoj84qivmqUk2MW3e6TVpY27tqBMxSHp3iKDcOu+cOkcf42/GB
# mOvNN7MOq2XTYuw6pXbrE6g1k8kuCgHswOjMPX626+LB7NMUkoJmh1Dc/VCXrLNK
# dnMGxIYROrNfQwRSb+qz0HQ2TMrxG3mEN3BjrXS5qg7zmLCGCOvb4B+MEPI5ZJuu
# TwoskopPGLWR5Y0ak18frvGm8C6X0NL2KzwxggOoMIIDpAIBATBAMCwxKjAoBgNV
# BAMTIVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdAIQ6d72z6XZs7ND
# Q2kZL8jnYjAJBgUrDgMCGgUAoHgwGAYKKwYBBAGCNwIBDDEKMAigAoAAoQKAADAZ
# BgkqhkiG9w0BCQMxDAYKKwYBBAGCNwIBBDAcBgorBgEEAYI3AgELMQ4wDAYKKwYB
# BAGCNwIBFTAjBgkqhkiG9w0BCQQxFgQUz9OoCE3HfJgrBnozxKIQrlng4qAwDQYJ
# KoZIhvcNAQEBBQAEgYChcMvq8/P7W1Q67tbIX+FMj8B7kgetnkNBGyog9oiddN7t
# 5B9kKBH2CTo6l6Zi90+CefiZM28Jm2eKkxWjODD3ZROVO5Qs7WTe7/xL60xGE//O
# 8neNGNZXuxmlkHe1EsLYa87y5jjMByUSsCdoEZjR5tA3scY+euAW/ec/Z+SnaqGC
# AkQwggJABgkqhkiG9w0BCQYxggIxMIICLQIBADCBqjCBlTELMAkGA1UEBhMCVVMx
# CzAJBgNVBAgTAlVUMRcwFQYDVQQHEw5TYWx0IExha2UgQ2l0eTEeMBwGA1UEChMV
# VGhlIFVTRVJUUlVTVCBOZXR3b3JrMSEwHwYDVQQLExhodHRwOi8vd3d3LnVzZXJ0
# cnVzdC5jb20xHTAbBgNVBAMTFFVUTi1VU0VSRmlyc3QtT2JqZWN0AhBHio77WeHY
# PwzhQtKihwe+MAkGBSsOAwIaBQCgXTAYBgkqhkiG9w0BCQMxCwYJKoZIhvcNAQcB
# MBwGCSqGSIb3DQEJBTEPFw0xNDA0MTExMjI2MTRaMCMGCSqGSIb3DQEJBDEWBBTv
# 6FG9y4tdNt2LXiBgOhCpNXX3uDANBgkqhkiG9w0BAQEFAASCAQAaBIPpxfnT0Yie
# JxHJP/696ZvYSzfGPfxtidflYFn5tAtQjHazkfVTUbwuLnZmXPaF9KKGhIip78or
# 738+qlqt9imsk88HvEfDy3bxaT/KssuUcJg3YiiGqtxE7D9RydOq/zcusFFwi90k
# Et0ZuuaueeRuH1xa5Gfi2u1XBXSpVJZJRwOFLQARhqs3NIguYsbEOeEvod9b+xH7
# jDIB8JPd154WHZBkUIdlV6HYofxAl1PnLh3YigLAf75wyVfsqTM4SAJs1NHfZ85T
# y3go2bVPxpPeA/v73BbQzVNTXwfNxpuuM1pDMevgAt7iFhb3YRVtkF3Af9mdb/d3
# x5pADvqB
# SIG # End signature block
