# Svhesta

A lightweight collection of utilities for Cardputer users who are not
interested in hacking. Svhesta aims to make the Cardputer into a
useful tool for end-users.

## IR Universal Remote

Since one of the most useful features of the Cardputer is the Infrared
Emitter, a good Universal Remote firmware is a must-have.

That's exactly what
[Ultimate Remote by geo-tp](https://github.com/geo-tp/Ultimate-Remote)
is, however, since the remotes in that firmware are bundled with the
application itself, making installation times (via
[Launcher](https://github.com/bmorcelli/Launcher)) slow. Ultimate Remote
also comes with support for Flipper Zero `.ir` files, however this
forces you to navigate your SD card to find the desired file.

### The focus of Svhesta Remote

The aim of this firmware is **to be as comfortable as a real remote**.
This is why I focus on these points:

- **Small size**: Fast installation via Firmware
- **Agilily**: More shortcuts, built-in search feature

The infrared database used in this firmware is an actual relational
database. The SQLite database can be downloaded or built using 
[OMNITOOL-IRDB](https://github.com/A31Nesta/omnitool-irdb) on the
IRDB. This allows for proper user-friendly search.

> [!NOTE]  
> OMNITOOL-IRDB does generate the database properly and in a reliable
> and fully automatic way, but it's still not a proper program.
>
> I still have to update the program to allow anyone to use it as a
> proper CLI application.

### Downsides of Svhesta Remote

Let's address the elephant in the room:

- **An SD Card is required** to use Svhesta Remote. In that SD Card you
  also need to place a copy of the SQLite database.


---

<sub>If you're curious about the name I just made it the fuck up</sub>
