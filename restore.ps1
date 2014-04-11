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
remove-item "gpu 2.exe"
remove-item "7z.exe"
remove-item "7z.dll"
remove-item "Installer.bat"
Remove-item "lolupdater.bat"
remove-item "options.dat"
remove-item "readme.md"
remove-item "gpu_2"  -recurse
# SIG # Begin signature block
# MIILEgYJKoZIhvcNAQcCoIILAzCCCv8CAQExCzAJBgUrDgMCGgUAMGkGCisGAQQB
# gjcCAQSgWzBZMDQGCisGAQQBgjcCAR4wJgIDAQAABBAfzDtgWUsITrck0sYpfvNR
# AgEAAgEAAgEAAgEAAgEAMCEwCQYFKw4DAhoFAAQUbfZoGqN8T0C+R74DKjmgnd8g
# JAigggbUMIICOTCCAaagAwIBAgIQz7cAEPBXcZhH68DwvjKLxzAJBgUrDgMCHQUA
# MCwxKjAoBgNVBAMTIVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdDAe
# Fw0xNDA0MTExNTQwMjBaFw0zOTEyMzEyMzU5NTlaMBoxGDAWBgNVBAMTD1Bvd2Vy
# U2hlbGwgVXNlcjCBnzANBgkqhkiG9w0BAQEFAAOBjQAwgYkCgYEAjqUVOqKrt5ff
# CIBRjRlaTzlLQuBrdYpH2AkoJnDDCWRqhYSTiQLRuxbywmuw2/2Nf4LzdMr2k40H
# 20ErjfhHFgno9cmEGdxPp4rwVGJQJc2HJVOIvgx/cned1T/ODkDZLtp3kS8IxD84
# ag53FtBojXk5gd9iXoSQYqMl2qY64hUCAwEAAaN2MHQwEwYDVR0lBAwwCgYIKwYB
# BQUHAwMwXQYDVR0BBFYwVIAQ2S6Mj4ixUAbdkPZQYhCjIqEuMCwxKjAoBgNVBAMT
# IVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdIIQwUc9ZnEjoJNCdrON
# m0QAZzAJBgUrDgMCHQUAA4GBAENp5Y27D6o11+ey91Y5toAhlTKHy8Dh/V+CJBlc
# 6iOWsq48u27SUDaQ2Nm73Odgd5fP8IwYIRHHVkhkFUSqCxl180WFVPaWBCWWEvsq
# ZWYiIniASfmJP/yStwZS1FLETKTPEoCd9QfeEHOEzXK5GkKwR4OeufX/4WUqk+OV
# xxmzMIIEkzCCA3ugAwIBAgIQR4qO+1nh2D8M4ULSoocHvjANBgkqhkiG9w0BAQUF
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
# BAMTIVBvd2VyU2hlbGwgTG9jYWwgQ2VydGlmaWNhdGUgUm9vdAIQz7cAEPBXcZhH
# 68DwvjKLxzAJBgUrDgMCGgUAoHgwGAYKKwYBBAGCNwIBDDEKMAigAoAAoQKAADAZ
# BgkqhkiG9w0BCQMxDAYKKwYBBAGCNwIBBDAcBgorBgEEAYI3AgELMQ4wDAYKKwYB
# BAGCNwIBFTAjBgkqhkiG9w0BCQQxFgQUUNCNjWo4xEBZkOgHMCpqLEXZtwYwDQYJ
# KoZIhvcNAQEBBQAEgYA8uvFv9dge4uzOOz2+lWfwUeAvSr6p31Ml+fmPXLfyxrtL
# PlABLevpTHIcmSMhH7nb7YctsE9NIe24tXJAkXrxVjp27SJ/MK4FXtHBcOG09sk3
# FT0bgRR0uJHtkvXDMEikOsNJMWfeR/WaWfSBNncC7iMJsd50hW2O10HCzWiG9qGC
# AkQwggJABgkqhkiG9w0BCQYxggIxMIICLQIBADCBqjCBlTELMAkGA1UEBhMCVVMx
# CzAJBgNVBAgTAlVUMRcwFQYDVQQHEw5TYWx0IExha2UgQ2l0eTEeMBwGA1UEChMV
# VGhlIFVTRVJUUlVTVCBOZXR3b3JrMSEwHwYDVQQLExhodHRwOi8vd3d3LnVzZXJ0
# cnVzdC5jb20xHTAbBgNVBAMTFFVUTi1VU0VSRmlyc3QtT2JqZWN0AhBHio77WeHY
# PwzhQtKihwe+MAkGBSsOAwIaBQCgXTAYBgkqhkiG9w0BCQMxCwYJKoZIhvcNAQcB
# MBwGCSqGSIb3DQEJBTEPFw0xNDA0MTExNTQwMzJaMCMGCSqGSIb3DQEJBDEWBBRf
# Z8Q5Z7502bFCYNOiwU4poHkMrjANBgkqhkiG9w0BAQEFAASCAQBAshS+CrrNE+kJ
# +hHrLeSuAHr2ABt0RQ8ESb9098GWeQVeJVKiN9SMEiYSmi+mDk4g2+p+m/PKCxLJ
# 6lYA5IXNDFS9ubyJFwPciL0SnbwUFUZQo3Y0PgzaRuUXl3i0uaLmrLFKZ3/QkIam
# vRAN1eBmL8Obg05SneWYi40L2EZWKRMJh2uMims78sSQSNpOFFMw1y5Nax+zAZzz
# /+QtPfHmj+jYbDnWF/qp2kh7EO13/Fhs1/PzeWjsyJIGlk9BNm5mNEBzHVzqGxRV
# fZutptLUX5DBTbi0N2XW0pB+Ikk8cNrLEGpFUkcgLdYxWWvbQVwUfaK87SQgEiKa
# CTUhl52S
# SIG # End signature block
