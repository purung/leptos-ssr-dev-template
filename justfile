dev:
	zellij run -f -- direnv exec . cargo-leptos watch --hot-reload

pg:
	zellij run -f -- direnv exec . docker compose up 
