FROM wishlist-backend-build as build

COPY . .

RUN PATH=$PATH:$HOME/.cargo/bin scripts/build.sh
RUN cd frontend && npm run build

FROM debian:buster-slim 

WORKDIR /usr/app
COPY --from=build /build/target/release/app .
COPY --from=build /build/frontend/app/public .
COPY --from=build /build/.env .

CMD ["./app"]

