[Service]
Description=SigmaOS hardware setup
After=systemd-udev-settle.service
Before=sysinit.target

[Install]
WantedBy=sysinit.target
