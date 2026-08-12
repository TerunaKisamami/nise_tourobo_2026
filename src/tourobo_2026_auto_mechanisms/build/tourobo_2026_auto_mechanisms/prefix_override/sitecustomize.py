import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/hatsu/robobobo/nise_tourobo_2026/src/tourobo_2026_auto_mechanisms/install/tourobo_2026_auto_mechanisms'
