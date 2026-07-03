import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/hatsu/Robobobo/tourobo_2026/install/tourobo_auto_main_pkg'
