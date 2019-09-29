import createSagaMiddleware from 'redux-saga';
import { createStore, compose, applyMiddleware } from 'redux';
import rootSaga from './rootSaga';
import reducers from '../reducers';

/** Saga Middleware */
const sagaMiddleware = createSagaMiddleware();

/** Create middlewares for redux */
let middlewares = applyMiddleware(sagaMiddleware);

/** Create redux store */
const store = createStore(
  reducers,
  compose(middlewares)
);

/** run saga watchers */
sagaMiddleware.run(rootSaga);

export default store;
