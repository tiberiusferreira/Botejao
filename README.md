# Botejao
A telegram bot which sends UNICAMP's restaurant menu to a Telegram group.

This is mainly a "toy" project to teach me Rust and get people involved with Telegram. 

It uses Docker to make reliable deployments to a RaspberryPi 3 board.

Dockerfile.build installs the system dependencies used and the Dockerfile.run gets the geckodriver and compiles the actual code.

This way it is possible to update geckodriver and Botejao's code without having to recreate the whole image.


Planned features:

- [ ] Support for USP 
- [ ] Support for UFRGS
- [ ] When the menu changes during the day, highlight the changes
- [ ] Allow other groups to enable daily broadcasts of the Menu and changes throughout the day
