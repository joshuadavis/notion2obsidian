# Samba / Ubuntu

Check Samba version:

```bash
sudo samba --version
```

- For TimeMachine, you need Samba 4.8+

List samba users:

```bash
pdbedit -L -v
```

Restart samba server:

```bash
systemctl restart smbd
```

Samba logs:

```bash
less /var/log/samba/log.smbd
```

Samba config file: `/etc/samba/smb.conf`

Test config file `testparm -s`