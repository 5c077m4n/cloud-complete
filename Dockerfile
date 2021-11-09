FROM node:lts-alpine AS build

WORKDIR /usr/src/app

COPY package.json ./
RUN yarn install

COPY . .
RUN yarn run build


FROM node:lts-alpine
RUN apk add dumb-init

ARG NODE_ENV=production
ENV NODE_ENV=${NODE_ENV}

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/dist ./

USER node
CMD ["dumb-init", "yarn", "node", "./main"]
