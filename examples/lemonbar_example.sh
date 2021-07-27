while :; do
  echo "%{l}$(xdotool getwindowname $(xdotool getwindowfocus))%{r}$(date +'%a %b %d %Y | %T')"
done | lemonbar -p
