import * as React from 'react';
import * as ReactDOM from 'react-dom'
import registerServiceWorker from './registerServiceWorker'

// redux-first-router imports
import {connectRoutes} from 'redux-first-router'
import {applyMiddleware, combineReducers, compose, createStore} from 'redux'
import {Provider, connect, Dispatch} from 'react-redux'
import createHistory from 'history/createBrowserHistory'

import {Main} from './main'
import {MainState, Shape} from './interfaces'

const shapes = new Map()

const initialState: MainState = {
    shapes: shapes
}

const mainReducer = (state: MainState=initialState, action: any) => {
  // Misc config variables not related to the current schedule.
  // todo figure out how to add types to these
  switch (action.type) {
      case 'addBooks':
          return {...state}
      default:
          return state
  }
}

// Routing code here:
// https://github.com/faceyspacey/redux-first-router
const history = createHistory()

const routesMap = {
HOME: '/',      // action <-> url path
ABOUT: '/about'
}

const {reducer, middleware, enhancer} = connectRoutes(history, routesMap) // yes, 3 redux aspects

const rootReducer = combineReducers({location: reducer, main: mainReducer})
const middlewares = applyMiddleware(middleware)
// note the order: enhancer, then middlewares
const store = createStore(rootReducer, compose(enhancer, middlewares))

// Connext the redux store to React.
const mapStateToProps = (state: MainState) => ({ state: state })
const mapDispatchToProps = (dispatch: Dispatch<any>) => ({ dispatch: dispatch })

const Connected = connect(mapStateToProps, mapDispatchToProps)(Main)

ReactDOM.render(
  <Provider store={store}>
      <Connected />
  </Provider>,
  document.getElementById('root') as HTMLElement
)
registerServiceWorker()
