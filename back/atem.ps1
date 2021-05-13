add-type -path './8_5_3/SwitcherLib.dll'
$Global:atem = New-Object SwitcherLib.Switcher("192.168.0.40")
$atem.Connect()
$atem.GetSwitcherLibVersion()

$ME = $atem.GetMEs()
$uri = 'http://localhost:8000/api/atem'
while ($true) {
    $data = "{`"air`": $($me[0].Program), `"preview`": $($me[0].Preview)}"
    Invoke-WebRequest -Uri $uri -Body $data -ContentType 'application/json' -Method POST -UseBasicParsing
    Start-Sleep -Milliseconds 10  
} 