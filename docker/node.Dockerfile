FROM node:lts-alpine AS build

ENV NODE_ENV=production
ENV SKIP_POSTINSTALL=1
ARG BUILD_CONTEXT=back

RUN apk update && apk add --no-cache dumb-init

WORKDIR /usr/src/app

COPY .yarn/ .yarn/
COPY .yarnrc.yml ./
COPY services/$BUILD_CONTEXT/package.json ./

RUN yarn

COPY services/$BUILD_CONTEXT/ ./
RUN yarn build

USER node
CMD ["dumb-init", "yarn", "run", "start:prod"]
