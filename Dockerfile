# build stage
FROM rust:latest
WORKDIR /app

# Copy the source code
COPY . .
# Install dependencies
RUN cargo build --release

EXPOSE 3278

# set the start command to run the server
CMD ["cargo", "run", "--release", "--bin", "server"]