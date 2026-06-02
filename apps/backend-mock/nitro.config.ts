import errorHandler from './error';

process.env.COMPATIBILITY_DATE = new Date().toISOString();

const isDev = process.env.NODE_ENV === 'development';

export default defineNitroConfig({
  devErrorHandler: errorHandler,
  errorHandler: '~/error',

  "buildDir": ".nitro",
  // 仅 build 时生效，输出到 .output
  output: isDev
      ? undefined
      : {
        dir: '.output',
        serverDir: '.output/server',
        publicDir: '.output/public',
      },

  routeRules: {
    '/api/**': {
      cors: true,
      headers: {
        'Access-Control-Allow-Credentials': 'true',
        'Access-Control-Allow-Headers':
          'Accept, Authorization, Content-Length, Content-Type, If-Match, If-Modified-Since, If-None-Match, If-Unmodified-Since, X-CSRF-TOKEN, X-Requested-With',
        'Access-Control-Allow-Methods': 'GET,HEAD,PUT,PATCH,POST,DELETE',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Expose-Headers': '*',
      },
    },
  },


});
