$path = "kernel\core\SovereignMicro.cpp"
$content = Get-Content $path
$content = $content -replace 'void micro_dispatch\(void\* context\)', 'void micro_dispatch(void* context) { (void)context;'
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignNetStack.cpp"
$content = Get-Content $path
$content = $content -replace 'void netstack_inject\(void\* payload\)', 'void netstack_inject(void* payload) { (void)payload;'
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignPersonalization.cpp"
$content = Get-Content $path
$content = $content -replace 'void personalization_trigger\(uint32_t event_id\)', 'void personalization_trigger(uint32_t event_id) { (void)event_id;'
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignVFS.cpp"
$content = Get-Content $path
$content = $content -replace 'void vfs_sync_shard\(void\* data\)', 'void vfs_sync_shard(void* data) { (void)data;'
Set-Content -Path $path -Value $content
