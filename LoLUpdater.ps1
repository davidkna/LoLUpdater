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
Start-BitsTransfer http://labsdownload.adobe.com/pub/labs/flashruntimes/air/air13_win.exe
Start-BitsTransfer https://www.bugsplatsoftware.com/files/BugSplatNative.zip
start-process Cg-3.1_April2012_Setup.exe /silent -Wait
start-process dxwebsetup.exe -Wait
start-process air13_win.exe -Wait
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
# SIG # Begin signature block
# MIILEgYJKoZIhvcNAQcCoIILAzCCCv8CAQExCzAJBgUrDgMCGgUAMGkGCisGAQQB
# gjcCAQSgWzBZMDQGCisGAQQBgjcCAR4wJgIDAQAABBAfzDtgWUsITrck0sYpfvNR
# AgEAAgEAAgEAAgEAAgEAMCEwCQYFKw4DAhoFAAQUTryWLjI6jQ7wVdTEiZD6iO/x
# QEagggbUMIICOTCCAaagAwIBAgIQi5HS4/r9cYZI2kfVf0NMHzAJBgUrDgMCHQUA
# MCwxKjAoBgNVBAMTIVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdDAe
# Fw0xNDA0MTAxNzM5NTFaFw0zOTEyMzEyMzU5NTlaMBoxGDAWBgNVBAMTD1Bvd2Vy
# U2hlbGwgVXNlcjCBnzANBgkqhkiG9w0BAQEFAAOBjQAwgYkCgYEAt9JDD/TYQeMS
# 0E0Orzlkjpls7nh5gVArSvE/Ulbt4vj2npWt+k1z0AxXnGPl2L6b/3mo7Zc1f+Sy
# KX8WPFjSy40ePadj3X2dRPmLgH1XufnZpehCwHJA9eq39J0Bz9z2TrKm51LbcXMF
# DICmd8YpJ2E8HTPw6CUzj7NcTU8bkxECAwEAAaN2MHQwEwYDVR0lBAwwCgYIKwYB
# BQUHAwMwXQYDVR0BBFYwVIAQPZyNJyfnJuEpVkkcIHSRQaEuMCwxKjAoBgNVBAMT
# IVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdIIQwQuAzw+t7pRIGkzU
# erh/JjAJBgUrDgMCHQUAA4GBAAq+UlDnXWxnDB71L6LIeS2Et6AvBuRnk+uVN0Do
# YA2usLNw4g5SnU3XvMOEthFVUAOnjeMvZp3dRNZqXwzYYac5dgJibUmi8T2sTnHr
# oRuH26wfb2p0wmvanW8cK22UHfSAkp+sk5aEMsZtlkQaegYcyfVMXwVU3Z44Nzk9
# m0uzMIIEkzCCA3ugAwIBAgIQR4qO+1nh2D8M4ULSoocHvjANBgkqhkiG9w0BAQUF
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
# BAMTIVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdAIQi5HS4/r9cYZI
# 2kfVf0NMHzAJBgUrDgMCGgUAoHgwGAYKKwYBBAGCNwIBDDEKMAigAoAAoQKAADAZ
# BgkqhkiG9w0BCQMxDAYKKwYBBAGCNwIBBDAcBgorBgEEAYI3AgELMQ4wDAYKKwYB
# BAGCNwIBFTAjBgkqhkiG9w0BCQQxFgQUzRq14wiw4j2TRoxceZTed4Vfag8wDQYJ
# KoZIhvcNAQEBBQAEgYA5RTRCmin4pHmVWvLeUxQ2fQT3Dp+vslNbqfJqF1BmX88p
# aLyLbRWdpzcUFtSexx/62j2u33+r/StsGbzQUQeagzswbqQ0oHOS1/70QOhWd9bP
# vcRB3jaTKpn1eiFjQUqcNdGk18wjNgtQp29zX/oSgwaAkLxpu4eTblih3YGSgaGC
# AkQwggJABgkqhkiG9w0BCQYxggIxMIICLQIBADCBqjCBlTELMAkGA1UEBhMCVVMx
# CzAJBgNVBAgTAlVUMRcwFQYDVQQHEw5TYWx0IExha2UgQ2l0eTEeMBwGA1UEChMV
# VGhlIFVTRVJUUlVTVCBOZXR3b3JrMSEwHwYDVQQLExhodHRwOi8vd3d3LnVzZXJ0
# cnVzdC5jb20xHTAbBgNVBAMTFFVUTi1VU0VSRmlyc3QtT2JqZWN0AhBHio77WeHY
# PwzhQtKihwe+MAkGBSsOAwIaBQCgXTAYBgkqhkiG9w0BCQMxCwYJKoZIhvcNAQcB
# MBwGCSqGSIb3DQEJBTEPFw0xNDA0MTAxODIzMDZaMCMGCSqGSIb3DQEJBDEWBBSy
# /bR/76jQO6rq6l6Ysn+VUTIJ6zANBgkqhkiG9w0BAQEFAASCAQAHE4d0rJcFPX88
# GqtH6kUd5irJDzS/CXDPWSHk4fhMjJTHOjvwKylCLTXzNdS92Q+JcNR/NGKQTNKg
# ARfxxTN0GTyQVzTay4Ma2hdFTUon9z1p4MscaH4/0uPUg8QiCdrEXfimPdmY5o//
# VGYrbZa21Mp+yhv+OiSnRz0/6oP2l+qTpmZ+s3KJ5xotJaSz3W0YSc3EHwuJ+TmF
# dtRZTa6/ABEUFWIjRPuwgUM5Xj7J+x41HKYCE+NX8b5uqAbI50hhkgXR/Ru31SfL
# 1rE6htFunZWjAIXwXJrkcuV4tdrqP7NqsllV/2p/lCTsG9FIQ815GAfbj2d4EzlG
# TgaO7SZr
# SIG # End signature block
