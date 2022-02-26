# `rn` #

Utility for displaying the current time using the [Misalian Seximal Units](https://www.seximal.net/units) with Kunimunean Extensions.

## Usage ##
```sh
$ echo `rn` `rn -b` `rn -s` && 
15:43:01.1 1543011 154
$ rn 8:24:36
20:34:05.0
```

`rn` also supports using the system time zone instead of UTC using the `-l`/`--local` flag:

```sh
$ echo `rn -l` `rn -lb` `rn -ls`
55:43:01.1 5543011 554
```

### Formats ###
`rn` currently supports three output formats: snapshot form, extended snapshot form, and span form. Each output format is in base six.

#### Snapshot Form ####
snapshot form `-b`/`--basic` is the number of snaps that have elapsed since midnight, zero-padded to take up seven digits. There are two hundred and seventy-nine thousand, nine hundred and thirty-six snaps in a day. Because of how the units are specified, removing the delimiters from extended snapshot form produces snapshot form.

```sh
$ rn --basic 8:24:36
2034050
```

#### Extended Snapshot Form (Default) ####
Extended snapshot form is the default output format. It is printed as a string of the format `lp:ll:mt.sn`, where:

- `lp`: The current _lapse_. There are thirty-six lapses in a day. Zero-padded to two digits.
- `ll`: The current _lull_. There are thirty-six lulls in a lapse. Zero-padded to two digits.
- `mt`: The current _moment_. There are thirty-six moments in a lull. Zero-padded to two digits.
- `sn`: The current _snap_. There are six snaps in a moment. One digit.

```sh
$ rn 8:24:36
20:34:05.0
```

#### Span Form ####
Span form `-s`/`--span` is the number of spans that have elapsed since midnight, zero-padded to take up three digits. There are two hundred and sixteen spans in a day. Because of how the units are specified, the three digits of span form are also the first three digits of extended snapshot form; that is, `20:34:05.0` is within span `203`.

```sh
$ rn --span 8:24:36
203
```

## Acknowledgements ##
The Misalian Seximal Units were devised by [jan&nbsp;Misali](https://www.seximal.net), with extensions by Justin Kunimune. The snapshot and span forms are based on the formats used by the iOS&nbsp;app [seximal](https://github.com/thisIsTheFoxe/seximal).
