#!/bin/sh

# create test data folder
if [ ! -d data ]
  then mkdir data
  else echo "data directory already exists"
fi

# create test client data
if [ ! -f data/clients.csv ]
  then echo "0,FNAME,LNAME,newemail@address.com,true" > data/clients.csv
    echo "1,Timmy,Tester,tester@address.com,true" >> data/clients.csv
    echo "2,Big,Bad,Wolf@address.com,true" >> data/clients.csv
  else echo "data/clients.csv already exists"
fi

# create test employee data
if [ ! -f data/employees.csv ]
  then echo "0,EMPFN,EMPLN,empe@address.com,manager,true" > data/employees.csv
    echo "1,123,456,789@address.com,worker,true" >> data/employees.csv
    echo "2,753,159,8246@address.com,worker,true" >> data/employees.csv
  else echo "data/employees.csv already exists"
fi

# create test exports data
if [ ! -f data/exports.csv ]
  then echo "0,Client Export" > data/exports.csv
    echo "1,Employee Export" >> data/exports.csv
  else echo "data/exports.csv already exists"
fi

# create test jobs data
if [ ! -f data/jobs.csv ]
  then echo "0,Client Export,Sun Jan 01 2024 00:00:00 GMT+1000 (Australian Eastern Standard Time),Success" > data/jobs.csv
  else echo "data/jobs.csv already exists"
fi

# create test settings data
if [ ! -f data/settings.csv ]
  then echo "enable-dashboard,true" > data/settings.csv
    echo "enable-clients,false" >> data/settings.csv
    echo "enable-employees,false" >> data/settings.csv
    echo "enable-exporters,false" >> data/settings.csv
  else echo "data/settings.csv already exists"
fi
