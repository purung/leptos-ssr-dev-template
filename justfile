dev:
	zellij run -f -- direnv exec . cargo-leptos watch --hot-reload -v

pg:
	zellij run -f -- direnv exec . docker compose up 
