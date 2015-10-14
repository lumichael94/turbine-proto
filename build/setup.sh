#!/bin/sh

sudo apt-get install postgresql postgresql-contrib

cd /tmp

sudo -u postgres psql -U postgres -d postgres -c "alter user postgres with password 'api';"

sudo -u postgres createdb turbine 2> /dev/null || echo "Database already exists."

echo "Setup finished."
