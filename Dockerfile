FROM archlinux/base

# dependencies
RUN pacman -Suy --noconfirm rustup make gcc pkgconf sqlite \
python python-sqlalchemy yarn google-api-python-client google_auth_oauthlib apiclient
RUN rustup install nightly

# workdir
WORKDIR /cw

# copy to workdir
COPY . .

# build
RUN make

# expose ports
EXPOSE 8000

# run cmd
CMD ["cargo", "run", "--release"]
