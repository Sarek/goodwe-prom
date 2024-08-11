# GoodWe-Prom

A Rust-based service that grabs metric data from a GoodWe solar inverter
and provides them in a form that can be scraped by Prometheus.

In addition, it can be used to discover and identify GoodWe inverters in
the local network, and can also print the metrics on standard output.

In contrast to other existing solutions, `goodwe-prom` will request the
current state of the metrics whenever Prometheus attempts to scrape.
This way, the frequency of samples is only dependent on the Prometheus
configuration. The time spent requesting the data from the interverter
via Modbus is thus also a part of the overall scrape time reported by
Prometheus and not an indication of a slow service.

# Supported Devices

This software was only tested with a GoodWe GW20K-ET, but other models in
the ET series, and possibly also others, should be mostly supported.

If you have further information on which Modbus registers represent which
values, and would like to see them supported, please open a ticket.
