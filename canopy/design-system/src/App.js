/**
 * Copyright 2019 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import React from 'react';
import {
  BrowserRouter as Router,
  Route,
  Switch,
  Redirect
} from 'react-router-dom';

import './App.scss';

import SideNav from './components/navigation/SideNav';
import Overview from './views/Overview';
import Design from './views/Design';
import Components from './views/Components';

const tabs = [
  {
    name: 'Overview',
    route: '/overview'
  },
  {
    name: 'Design',
    route: '/design'
  },
  {
    name: 'Components',
    route: '/components'
  }
];

function App() {
  return (
    <Router>
      <div className="App">
        <SideNav tabs={tabs} />
        <div className="view">
          <Switch>
            <Redirect exact from="/" to="/overview" />
            <Route path="/overview">
              <Overview />
            </Route>
            <Route path="/design">
              <Design />
            </Route>
            <Route path="/components">
              <Components />
            </Route>
          </Switch>
        </div>
      </div>
    </Router>
  );
}

export default App;
