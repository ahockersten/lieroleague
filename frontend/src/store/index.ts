import ky from 'ky';
import { createLogger } from 'redux-logger';
import createSagaMiddleware from 'redux-saga';
import { createStore, applyMiddleware } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';
import rootSaga from './rootSaga';
import reducers from '../reducers';

/** Saga Middleware */
const sagaMiddleware = createSagaMiddleware();

/** Create middlewares for redux */
const middlewares = applyMiddleware(sagaMiddleware, createLogger());

/** Create redux store */
const store = createStore(reducers, composeWithDevTools(middlewares));

/** run saga watchers */
sagaMiddleware.run(rootSaga);

export const apiClient = ky.extend({
  prefixUrl: 'http://localhost:8000',
  credentials: 'include' // FIXME somewhat dangerous?
});

export default store;
