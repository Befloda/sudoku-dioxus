#!/bin/bash
# Lance Tailwind en arrière-plan pour surveiller les changements dans les fichiers CSS
tailwindcss -i ./src/styles.tw4.css -o ./src/tailwind.css --watch &
# Lance Dioxus
dx serve --web