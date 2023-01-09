# How to Setup a DNS Server for a Home Lab on Ubuntu 14.04

Created: March 27, 2020 9:15 PM
URL: http://techpolymath.com/2015/02/16/how-to-setup-a-dns-server-for-a-home-lab-on-ubuntu-14-04/

While it is not essential to have a private DNS server for your home lab I’ve found that many scenarios are rather difficult without one. Using fully qualified domain names rather than IP addresses makes configuring applications and infrastructure services easier. Even from an administrative perspective remembering names is clearly more efficient than four octets.

In this post, we will setup a private DNS environment consisting of primary and secondary servers running BIND (BIND9). Both will run Ubuntu Server 14.04 as virtual machines on a VMware ESXi 5.5 host. We’ll also configure a third Ubuntu server that will act as a client to test querying the new DNS servers.

## Prerequisites

- VM to serve as the Primary DNS server, ns1 (1 vCPU, 1GB vRAM)
- VM to serve as the Secondary DNS server, ns2 (1 vCPU, 1GB vRAM)
- VM to test querying the DNS servers, host1 (1 vCPU, 1GB vRAM)

Parameters used in this example:

- `homelab.local` will be used as the lab domain name
- 10.1.100.0/24 is the private subnet

[Untitled](How%20to%20Setup%20a%20DNS%20Server%20for%20a%20Home%20Lab%20on%20Ubuntu%20bfee0474ac0345ab9c6fcce76ac20d63/Untitled%20Database%20240461ceab7040889b2171196ea67c0d.csv)

## Configure Primary Server Networking

In order for the servers to reach the Ubuntu package repositories you need to edit the network interfaces configuration file and hosts file.

Edit the network interfaces configuration file on the ns1 server:

```
$ sudo vi /etc/network/interfaces

```

When done editing it should look like:

```
iface lo inet loopback  
auto lo

# primary network interface
auto eth0  
iface eth0 inet static  
        address 10.1.100.41
        netmask 255.255.255.0
        network 10.1.100.0
        broadcast 10.1.100.255
        gateway 10.1.100.1
        dns-nameservers 8.8.8.8 8.8.4.4

```

Then restart networking:

```
$ /etc/init.d/networking restart

```

Edit the hosts file on the ns1 server:

```
$ sudo vi /etc/hosts

```

When done editing it should look like:

```
127.0.0.1    localhost

# The following lines are desirable for IPv6 capable hosts
::1     localhost ip6-localhost ip6-loopback
ff02::1 ip6-allnodes  
ff02::2 ip6-allrouters

10.1.100.41    ns1.homelab.local ns1  

```

## Configure Secondary Server Networking

Repeat the same process for the secondary server, ns2.

Edit the network interfaces configuration file on the ns2 server:

```
$ sudo vi /etc/network/interfaces

```

When done editing it should look like:

```
iface lo inet loopback  
auto lo

# primary network interface
auto eth0  
iface eth0 inet static  
        address 10.1.100.42
        netmask 255.255.255.0
        network 10.1.100.0
        broadcast 10.1.100.255
        gateway 10.1.100.1
        dns-nameservers 8.8.8.8 8.8.4.4

```

Then restart networking:

```
$ /etc/init.d/networking restart

```

Edit the hosts file on the ns2 server:

```
$ sudo vi /etc/hosts

```

When done editing it should look like:

```
127.0.0.1    localhost

# The following lines are desirable for IPv6 capable hosts
::1     localhost ip6-localhost ip6-loopback
ff02::1 ip6-allnodes  
ff02::2 ip6-allrouters

10.1.100.42    ns2.homelab.local ns2  

```

## Install BIND on the Primary DNS Server

Connect to the ns1 host (10.1.100.41) via SSH.

Download package lists and information of latest versions:

```
$ sudo apt-get -y update

```

Install BIND packages:

```
$ sudo apt-get -y install bind9 bind9utils bind9-doc

```

Configure BIND to run in IPv4 mode by editing the bind9 service parameters file:

```
$ sudo vi /etc/default/bind9

```

Add “-4” to the OPTIONS variable. It should look like the following:

```
# run resolvconf?
RESOLVCONF=no

# startup options for the server
OPTIONS="-4 -u bind"  

```

Save the file and exit.

## Configure the Primary DNS Server

The primary configuration file for the BIND DNS server named process is `/etc/bind/named.conf`. It includes three additional configuration files: `named.conf.options`, `named.conf.local`, and `named.conf.default-zones`.

### Configure Options File

On ns1, edit the named.conf.options file:

```
$ sudo vi /etc/bind/named.conf.options

```

By default the file should look like the following:

```
options {  
        directory "/var/cache/bind";

        // If there is a firewall between you and nameservers you want
        // to talk to, you may need to fix the firewall to allow multiple
        // ports to talk.  See http://www.kb.cert.org/vuls/id/800113

        // If your ISP provided one or more IP addresses for stable
        // nameservers, you probably want to use them as forwarders.
        // Uncomment the following block, and insert the addresses replacing
        // the all-0's placeholder.

        // forwarders {
        //      0.0.0.0;
        // };

        //========================================================================
        // If BIND logs error messages about the root key being expired,
        // you will need to update your keys.  See https://www.isc.org/bind-keys
        //========================================================================
        dnssec-validation auto;

        auth-nxdomain no;    # conform to RFC1035
        listen-on-v6 { any; };
};

```

First we will define an access control list called `trusted` that will determine which clients the DNS servers will allow recursive queries from. You can enter individual client IP addresses to tightly control access but since this is a home lab we will use subnets to permit recursive queries from the 10.1.100.0/24 network. Add the ACL before the `options` block as follows:

```
// Lab subnets we wish to allow recursive queries from.
acl "trusted" {  
        10.1.100.0/24;   # lab network
};

```

Edit the `directory` directive to read as follows:

```
options {  
        directory "/var/cache/bind";

        recursion yes;                 # enables resursive queries
        allow-recursion { trusted; };  # allows recursive queries from "trusted" clients
        listen-on { 10.1.100.41; };    # ns1 private IP address - listen on private network only
        allow-transfer { none; };      # disable zone transfers by default

        forwarders {
                8.8.8.8;
                8.8.4.4;
        };

        dnssec-validation auto;

        auth-nxdomain no;    # conform to RFC1035
        listen-on-v6 { any; };
};

```

Save the file and exit the editor.

### Configure Local DNS Zones

We will define our local forward and reserve DNS zones in the `named.conf.local` file.

On ns1, open the file for editing:

```
$ sudo vi /etc/bind/named.conf.local

```

Add a forward zone for `homelab.local`:

```
zone "homelab.local" {  
    type master;
    file "/etc/bind/zones/db.homelab.local"; # zone file path
    allow-transfer { 10.1.100.42; };         # ns2 private IP address – secondary
};

```

Add a reverse zone for the 10.1.100.0/24 subnet. The reverse mapping for 10.1.100 is entered as `100.1.10`:

```
zone "100.1.10.in-addr.arpa" {  
    type master;
    file "/etc/bind/zones/db.10.1.100";  # 10.1.100.0/24 subnet
    allow-transfer { 10.1.100.42; };     # ns2 private IP address – secondary
};

```

If your lab includes multiple subnets you want to provide DNS resolution for you should add a zone and zone file for each subnet. At this point the `/etc/bind/named.conf.local` file should look like the following:

```
zone "homelab.local" {  
    type master;
    file "/etc/bind/zones/db.homelab.local"; # zone file path
    allow-transfer { 10.1.100.42; };         # ns2 private IP address – secondary
};

zone "100.1.10.in-addr.arpa" {  
    type master;
    file "/etc/bind/zones/db.10.1.100";  # 10.1.100.0/24 subnet
    allow-transfer { 10.1.100.42; };     # ns2 private IP address – secondary
};

```

Save the file and exit the editor.

### Create the Forward Zone File

Once the zones are specified in the BIND configuration file we need to create forward and reverse zone files. We will start with the forward zone file which defines DNS records for forward lookups. For example, if a client queries the DNS server for `host1.homelab.local` the server will look in the `homelab.local` forward zone file for a record mapping `host1` to it’s IP address.

The forward zone file is where we define DNS records for forward DNS lookups. That is, when the DNS receives a name query, “host1.homelab.local” for example, it will look in the forward zone file to resolve host1’s corresponding private IP address.

In the `named.conf.local` file the default zone file location was specified to be `/etc/bind/zones`. The directory must be created before we can store zone files:

```
$ sudo mkdir /etc/bind/zones

```

Create the forward zone file for `homelab.local` based on the `/etc/bind/db.local` sample:

```
$ cd /etc/bind/zones
$ sudo cp ../db.local ./db.homelab.local

```

Edit the forward zone file:

```
$ sudo vi /etc/bind/zones/db.homelab.local

```

By default, it should match the following:

```
;
; BIND data file for local loopback interface
;
$TTL    604800
@       IN      SOA     localhost. root.localhost. (
                              2         ; Serial
                         604800         ; Refresh
                          86400         ; Retry
                        2419200         ; Expire
                         604800 )       ; Negative Cache TTL
;
@       IN      NS      localhost.
@       IN      A       127.0.0.1
@       IN      AAAA    ::1

```

Edit the SOA record replacing `localhost` with ns1’s FQDN, replacing `root.localhost` with `admin.homelab.local`, and incrementing the `serial` value:

```
@       IN      SOA     ns1.homelab.local. admin.homelab.local. (
                              3         ; Serial

```

Delete the `localhost.`, `127.0.0.1`, and `::1` records.

Add nameserver (NS) records for the ns1 and ns2 servers:

```
; name servers - NS records
        IN      NS      ns1.homelab.local.
        IN      NS      ns2.homelab.local.

```

Add A records for the hosts in your lab that you want to have *.homelab.local FQDNs. For this example we’ll create A records for ns1, ns2, and host1:

```
; name servers - A records
ns1.homelab.local.          IN      A       10.1.100.41  
ns2.homelab.local.          IN      A       10.1.100.42  
;
; 10.1.100.0/24 - A records
host1.homelab.local.        IN      A       10.1.100.90  

```

At this point the file should look like the following:

```
$TTL    604800
@       IN      SOA     ns1.homelab.local. admin.homelab.local. (
                              3         ; Serial
                         604800         ; Refresh
                          86400         ; Retry
                        2419200         ; Expire
                         604800 )       ; Negative Cache TTL
;
; name servers - NS records
        IN      NS      ns1.homelab.local.
        IN      NS      ns2.homelab.local.
;
; name servers - A records
ns1.homelab.local.          IN      A       10.1.100.41  
ns2.homelab.local.          IN      A       10.1.100.42  
;
; 10.1.100.0/24 - A records
host1.homelab.local.        IN      A       10.1.100.90  

```

Save and exit the `db.homelab.local` file.

### Create the Reverse Zone File

Next we will create a reverse zone file containing DNS PTR records for reverse DNS lookups. For example, if a client queries the DNS server for `10.1.100.90` the server will look in the `10.1.100` zone file for a record mapping `10.1.100.90` to the FQDN `host1.homelab.local`.

In the `named.conf.local` file on ns1 the file for the reverse zone `100.1.10.in-addr.arpa` was set to be `/etc/bind/zones/db.10.1.100`. Create the reverse zone file based on the `/etc/bind/db.127` sample:

```
$ cd /etc/bind/zones
$ sudo cp ../db.127 ./db.10.1.100

```

Edit the reverse zone file:

```
$ sudo vi /etc/bind/zones/db.10.1.100

```

By default, it should match the following:

```
;
; BIND reverse data file for local loopback interface
;
$TTL    604800
@       IN      SOA     localhost. root.localhost. (
                              1         ; Serial
                         604800         ; Refresh
                          86400         ; Retry
                        2419200         ; Expire
                         604800 )       ; Negative Cache TTL
;
@       IN      NS      localhost.
1.0.0   IN      PTR     localhost.  

```

As we did with the forward zone file edit the SOA record and increment the `serial` value as follows:

```
@       IN      SOA     ns1.homelab.local. admin.homelab.local. (
                              2         ; Serial

```

Delete the `localhost.` NS and `localhost.` PTR records.

Add nameserver (NS) records for the ns1 and ns2 servers:

```
; name servers - NS records
        IN      NS      ns1.homelab.local.
        IN      NS      ns2.homelab.local.

```

Add PTR records for each of the hosts in your lab. The first column will be the last octet of the host’s IP addresses in reverse order. If you were using a /16 network then you would enter the last two octets of the host’s IP addresses in reverse order. For this example we’ll create records for the ns1, ns2, and host1 hosts on the 10.1.100.0/24 subnet:

```
; PTR Records
41      IN      PTR     ns1.homelab.local.    ; 10.1.100.41  
42      IN      PTR     ns2.homelab.local.    ; 10.1.100.42  
90      IN      PTR     host1.homelab.local.  ; 10.1.100.90  

```

At this point the file should look like the following:

```
$TTL    604800
@       IN      SOA     ns1.homelab.local. admin.homelab.local. (
                              2         ; Serial
                         604800         ; Refresh
                          86400         ; Retry
                        2419200         ; Expire
                         604800 )       ; Negative Cache TTL
;
; name servers - NS records
        IN      NS      ns1.homelab.local.
        IN      NS      ns2.homelab.local.
;
; PTR Records
41      IN      PTR     ns1.homelab.local.    ; 10.1.100.41  
42      IN      PTR     ns2.homelab.local.    ; 10.1.100.42  
90      IN      PTR     host1.homelab.local.  ; 10.1.100.90  

```

Save and exit the `db.10.1.100` file.

Repeat these steps for any additional subnets used in your lab.

### Check BIND Configuration File Syntax

Check the syntax of the configuration files that start with `named.conf`:

```
$ sudo named-checkconf

```

If the configuration files have no syntax errors you won’t see any error messages.

Check the syntax of the forward zone file:

```
$ sudo named-checkzone homelab.local db.homelab.local

```

If there are no syntax errors you should see something similar to the following:

```
zone homelab.local/IN: loaded serial 3  
OK  

```

Check the syntax of the reverse zone file:

```
$ sudo named-checkzone 100.1.10.in-addr.arpa /etc/bind/zones/db.10.1.100

```

If there are no syntax errors you should see something similar to the following:

```
zone 100.1.10.in-addr.arpa/IN: loaded serial 2  
OK  

```

Restart the BIND service:

```
$ sudo service bind9 restart

```

## Install BIND on the Secondary Server

Connect to the ns2 host (10.1.100.42) via SSH.

Download package lists and information of latest versions:

```
$ sudo apt-get -y update

```

Install BIND packages:

```
$ sudo apt-get -y install bind9 bind9utils bind9-doc

```

Configure BIND to run in IPv4 mode by editing the bind9 service parameters file:

```
$ sudo vi /etc/default/bind9

```

Add "-4" to the OPTIONS variable. It should look like the following:

```
# run resolvconf?
RESOLVCONF=no

# startup options for the server
OPTIONS="-4 -u bind"  

```

Save the file and exit.

## Configure the Secondary DNS Server

### Configure Options File

On ns2, edit the `named.conf.options` file:

```
$ sudo vi /etc/bind/named.conf.options

```

By default the file should look like the following:

```
options {  
        directory "/var/cache/bind";

        // If there is a firewall between you and nameservers you want
        // to talk to, you may need to fix the firewall to allow multiple
        // ports to talk.  See http://www.kb.cert.org/vuls/id/800113

        // If your ISP provided one or more IP addresses for stable
        // nameservers, you probably want to use them as forwarders.
        // Uncomment the following block, and insert the addresses replacing
        // the all-0's placeholder.

        // forwarders {
        //      0.0.0.0;
        // };

        //========================================================================
        // If BIND logs error messages about the root key being expired,
        // you will need to update your keys.  See https://www.isc.org/bind-keys
        //========================================================================
        dnssec-validation auto;

        auth-nxdomain no;    # conform to RFC1035
        listen-on-v6 { any; };
};

```

Add an ACL to permit recursive queries from the 10.1.100.0/24 network before the `options` block as follows:

```
// Lab subnets we wish to allow recursive queries from.
acl "trusted" {  
        10.1.100.0/24;   # lab network
};

```

Edit the directory directive to read as follows:

```
options {  
        directory "/var/cache/bind";

        recursion yes;                 # enables resursive queries
        allow-recursion { trusted; };  # allows recursive queries from "trusted" clients
        listen-on { 10.1.100.42; };    # ns2 private IP address - listen on private network only
        allow-transfer { none; };      # disable zone transfers by default

        forwarders {
                8.8.8.8;
                8.8.4.4;
        };

        dnssec-validation auto;

        auth-nxdomain no;    # conform to RFC1035
        listen-on-v6 { any; };
};

```

Save the file and exit the editor.

### Configure Local DNS Zones

We will define our local slave zones on the secondary DNS server that correspond to the master zones defined on the primary. As you will see below the `file` parameter for zones of type `slave` does not contain a path and there is a `masters` directive that is set to the IP address of the primary DNS server.

On ns2, open the file for editing:

```
$ sudo vi /etc/bind/named.conf.local

```

Add a forward zone for `homelab.local`:

```
zone "homelab.local" {  
    type slave;
    file "db.homelab.local";
    masters { 10.1.100.41; };  # ns1 private IP
};

```

Add a reverse zone for the 10.1.100.0/24 subnet. The reverse mapping for 10.1.100 is entered as `100.1.10`:

```
zone "100.1.10.in-addr.arpa" {  
    type slave;
    file "db.10.1.100";
    masters { 10.1.100.41; };  # ns1 private IP
};

```

The `/etc/bind/named.conf.local` file should look like the following:

```
zone "homelab.local" {  
    type slave;
    file "db.homelab.local";
    masters { 10.1.100.41; };  # ns1 private IP
};

zone "100.1.10.in-addr.arpa" {  
    type slave;
    file "db.10.1.100";
    masters { 10.1.100.41; };  # ns1 private IP
};

```

Save the file and exit the editor.

### Check BIND Configuration File Syntax

Check the syntax of the configuration files that start with `named.conf`:

```
$ sudo named-checkconf

```

If the configuration files have no syntax errors you won’t see any error messages.

Restart the BIND service:

```
$ sudo service bind9 restart

```

## Test DNS Using Ubuntu Client

The primary and secondary DNS servers have now been deployed, so it is time to test name and IP address resolution. We will use a third Ubuntu 14.04 Server configured to query our two new servers.

### Configure Test Client Networking

Connect to the host1 host (10.1.100.90) via SSH.  Edit the network interfaces configuration file:

```
$ sudo vi /etc/network/interfaces

```

Look for the parameters. Remove the existing `dns-nameservers` and `dns-search` entries and replace them with your private domain and ns1 and ns2 private IP addresses.  When done editing it should look like:

```
iface lo inet loopback  
auto lo

# primary network interface
auto eth0  
iface eth0 inet static  
        address 10.1.100.90
        netmask 255.255.255.0
        network 10.1.100.0
        broadcast 10.1.100.255
        gateway 10.1.100.1
        dns-search homelab.local
        dns-nameservers 10.1.100.41 10.1.100.42

```

Save and close the file.

Bounce the interface to apply the changes:

```
$ sudo ifdown eth0 && sudo ifup eth0

```

### Test Forward Lookup

Run the following command to perform a forward lookup and retrieve the IP address of host1.homelab.local:

```
$ nslookup host1

```

When you perform a DNS query for host1 it is expanded to `host1.homelab.local` because of the `dns-search homelab.local` that is set in the network interfaces configuration file. The command output should be:

```
frank@host1:~$ nslookup host1  
Server:        10.1.100.41  
Address:    10.1.100.41#53

Name:    host1.homelab.local  
Address: 10.1.100.90  

```

### Test Reverse Lookup

Run the following command to perform a reverse lookup of host1’s IP address:

```
$ nslookup 10.1.100.90

```

The command output should be:

```
Server:        10.1.100.41  
Address:    10.1.100.41#53

91.100.1.10.in-addr.arpa    name = host1.homelab.local.  

```

### Query NS1 Using DIG

Run the following command to query the primary DNS server using DIG:

```
$ dig homelab.local any @ns1.homelab.local

```

The command output should be:

```
; <<>> DiG 9.9.5-3ubuntu0.1-Ubuntu <<>> homelab.local any @ns1.homelab.local
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 20320
;; flags: qr aa rd ra; QUERY: 1, ANSWER: 3, AUTHORITY: 0, ADDITIONAL: 3

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 4096
;; QUESTION SECTION:
;homelab.local.            IN  ANY

;; ANSWER SECTION:
homelab.local.        604800  IN  SOA ns1.homelab.local. admin.homelab.local. 3 604800 86400 2419200 604800  
homelab.local.        604800  IN  NS  ns1.homelab.local.  
homelab.local.        604800  IN  NS  ns2.homelab.local.

;; ADDITIONAL SECTION:
ns1.homelab.local.    604800  IN  A   10.1.100.41  
ns2.homelab.local.    604800  IN  A   10.1.100.42

;; Query time: 1 msec
;; SERVER: 10.1.100.41#53(10.1.100.41)
;; WHEN: Fri Feb 13 22:31:58 EST 2015
;; MSG SIZE  rcvd: 152

```

### Query NS2 Using DIG

Run the following command to query the secondary DNS server using DIG:

```
$ dig homelab.local any @ns2.homelab.local

```

The command output should be:

```
; <<>> DiG 9.9.5-3ubuntu0.1-Ubuntu <<>> homelab.local any @ns2.homelab.local
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 58763
;; flags: qr aa rd ra; QUERY: 1, ANSWER: 3, AUTHORITY: 0, ADDITIONAL: 3

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 4096
;; QUESTION SECTION:
;homelab.local.            IN  ANY

;; ANSWER SECTION:
homelab.local.        604800  IN  NS  ns1.homelab.local.  
homelab.local.        604800  IN  NS  ns2.homelab.local.  
homelab.local.        604800  IN  SOA ns1.homelab.local. admin.homelab.local. 3 604800 86400 2419200 604800

;; ADDITIONAL SECTION:
ns1.homelab.local.    604800  IN  A   10.1.100.41  
ns2.homelab.local.    604800  IN  A   10.1.100.42

;; Query time: 1 msec
;; SERVER: 10.1.100.42#53(10.1.100.42)
;; WHEN: Fri Feb 13 22:32:04 EST 2015
;; MSG SIZE  rcvd: 152

```

If the command output from the forward and reverse lookups match the examples above it means that your DNS servers are configured properly.

## Summary

This post walks through setting up a private DNS environment with redundant DNS servers running BIND. We also tested querying the DNS servers using a third Ubuntu system to confirm that both work as expected. In a future post maintaining DNS records will be covered.