import React from 'react';
import { create, Workbench } from '@dtinsight/molecule';
import '@dtinsight/molecule/esm/style/mo.css';

const moInstance = create({
  extensions: [],
});

const App = () => moInstance.render(<Workbench />);

export default App;
