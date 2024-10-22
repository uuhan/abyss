#!/bin/bash
systemd-run --user --on-calendar="Mon..Fri *-*-* 20:30:00" -- systemctl --user restart cta
