# Direwolf IGate Telemetry

A small telemetry app for Direwolf to report temperature, load average and receive audio level to APRS.

## How to build and install

Clone this repo to your APRS IGate and run `cargo build --release` to build the app. The resulting binary is located at `target/release/telem-direwolf`.

Cross compilation is also supported: set up [cross](https://github.com/cross-rs/cross), run `cross build --target arm-unknown-linux-gnueabihf --release`, and copy `target/arm-unknown-linux-gnueabihf/release/telem-direwolf` to `/usr/local/bin/telem-direwolf` on your Raspberry Pi or any similar device.

## How to use

### Direwolf configuration

The app is intended to be used in `commentcmd` together with `telem-data91.pl`:

```
PBEACON sendto=IG delay=1:05 every=15:00 symbol="igate" overlay=R comment="1200bps RX only IGate" commentcmd="telem-data91.pl `telem-seq.sh` `/usr/local/bin/telem-direwolf`"
```

Telemetry metadata can be configured this way:

```
CBEACON sendto=IG delay=1:30 every=1000000 infocmd="telem-parm.pl N0CALL-10 Temp LoadAvg AudioLvl"
CBEACON sendto=IG delay=1:31 every=1000000 infocmd="telem-unit.pl N0CALL-10 C 5m %"
CBEACON sendto=IG delay=1:32 every=1000000 infocmd="telem-eqns.pl N0CALL-10 0 1 0 0 0.1 0"
```

### Audio level reporting

Receiving audio level is gathered from logs so first you need to enable audio level logging. Run `systemctl edit` and change `ExecStart` by appending `-a 60`:

```
[Service]
SupplementaryGroups=systemd-journal
ExecStart=
ExecStart=/usr/bin/direwolf -c /etc/direwolf.conf -r 24000 -a 60
StandardOutput=journal
```

Next, restart Direwolf:

```
systemctl daemon-reload
systemctl restart direwolf
```

And it should start reporting audio level to logs and APRS.