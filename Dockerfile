FROM node:12-alpine AS builder
WORKDIR /Build
COPY . /Build
RUN npm install
RUN npm run build

#Can't use alpine as fs.watch needs inotify
FROM node:12-slim
COPY --from=builder /Build/dist/. .
COPY --from=builder /Build/package-lock.json .
COPY --from=builder /Build/package.json .
RUN npm run ci
CMD ["node", "main.js"]