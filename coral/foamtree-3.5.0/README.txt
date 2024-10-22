
Carrot Search FoamTree
----------------------

Carrot Search FoamTree is an interactive visualization of hierarchical data
structures, such as groups of documents or network domains. It works very well
with documents or search results clustered using the Lingo3G document clustering
engine.


Contents of this package
------------------------

./carrotsearch.foamtree.js
  FoamTree implementation, required at runtime.

./carrotsearch.foamtree.asserts.js
  Option value validator, useful for development and debugging, not required at runtime.

./carrotsearch.foamtree.util.*.js
  Utility scripts that handle a number of typical programming tasks around FoamTree.

./api/
  Documentation and API examples.

./demos/
  More examples.

./tests/
  Automated functional tests of FoamTree.


Documentation
-------------

Open api/index.html in a modern browser (Chrome or Firefox recommended).

Examples are fully functional when served over the HTTP protocol.
Start up a local HTTP daemon (Java required):

  bin/nhttpd.sh  -p 8080     # (Linux, Mac)
  bin\nhttpd.bat -p 8080     # (Windows)

and open http://localhost:8080 in a browser.


Contact
-------

For further information and support contact Carrot Search: info@carrotsearch.com


Build information
-----------------

Build type    : Carrot Search FoamTree HTML5 (demo variant)
Build version : 3.5.0
Build number  : FOAMTREE-SOFTWARE5-DIST-3
Build time    : Jan 18, 2021
Built by      : bamboo
Build revision: bugfix/3.5.x/e3b91c8e


Carrot Search Confidential
Copyright 2002-2021 Carrot Search
