dev:
	zellij run -f -- direnv exec . cargo-leptos watch --hot-reload -vvv

pg:
	zellij run -f -- direnv exec . docker compose up 
