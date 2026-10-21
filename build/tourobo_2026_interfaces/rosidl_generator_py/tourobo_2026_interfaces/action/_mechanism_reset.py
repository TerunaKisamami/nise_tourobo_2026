# generated from rosidl_generator_py/resource/_idl.py.em
# with input from tourobo_2026_interfaces:action/MechanismReset.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_MechanismReset_Goal(type):
    """Metaclass of message 'MechanismReset_Goal'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_Goal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__goal
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__goal
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__goal
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__goal
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__goal

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_Goal(metaclass=Metaclass_MechanismReset_Goal):
    """Message class 'MechanismReset_Goal'."""

    __slots__ = [
        '_execute',
    ]

    _fields_and_field_types = {
        'execute': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.execute = kwargs.get('execute', bool())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.execute != other.execute:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def execute(self):
        """Message field 'execute'."""
        return self._execute

    @execute.setter
    def execute(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'execute' field must be of type 'bool'"
        self._execute = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_Result(type):
    """Metaclass of message 'MechanismReset_Result'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_Result')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__result
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__result
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__result
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__result
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__result

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_Result(metaclass=Metaclass_MechanismReset_Result):
    """Message class 'MechanismReset_Result'."""

    __slots__ = [
        '_success',
        '_next_state',
        '_next_carry',
        '_next_push_state',
        '_next_is_left_arm_open',
        '_next_is_right_arm_open',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'next_state': 'int32',
        'next_carry': 'int32',
        'next_push_state': 'int32',
        'next_is_left_arm_open': 'boolean',
        'next_is_right_arm_open': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.next_state = kwargs.get('next_state', int())
        self.next_carry = kwargs.get('next_carry', int())
        self.next_push_state = kwargs.get('next_push_state', int())
        self.next_is_left_arm_open = kwargs.get('next_is_left_arm_open', bool())
        self.next_is_right_arm_open = kwargs.get('next_is_right_arm_open', bool())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.success != other.success:
            return False
        if self.next_state != other.next_state:
            return False
        if self.next_carry != other.next_carry:
            return False
        if self.next_push_state != other.next_push_state:
            return False
        if self.next_is_left_arm_open != other.next_is_left_arm_open:
            return False
        if self.next_is_right_arm_open != other.next_is_right_arm_open:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def success(self):
        """Message field 'success'."""
        return self._success

    @success.setter
    def success(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'success' field must be of type 'bool'"
        self._success = value

    @builtins.property
    def next_state(self):
        """Message field 'next_state'."""
        return self._next_state

    @next_state.setter
    def next_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_state' field must be an integer in [-2147483648, 2147483647]"
        self._next_state = value

    @builtins.property
    def next_carry(self):
        """Message field 'next_carry'."""
        return self._next_carry

    @next_carry.setter
    def next_carry(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_carry' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_carry' field must be an integer in [-2147483648, 2147483647]"
        self._next_carry = value

    @builtins.property
    def next_push_state(self):
        """Message field 'next_push_state'."""
        return self._next_push_state

    @next_push_state.setter
    def next_push_state(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'next_push_state' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'next_push_state' field must be an integer in [-2147483648, 2147483647]"
        self._next_push_state = value

    @builtins.property
    def next_is_left_arm_open(self):
        """Message field 'next_is_left_arm_open'."""
        return self._next_is_left_arm_open

    @next_is_left_arm_open.setter
    def next_is_left_arm_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'next_is_left_arm_open' field must be of type 'bool'"
        self._next_is_left_arm_open = value

    @builtins.property
    def next_is_right_arm_open(self):
        """Message field 'next_is_right_arm_open'."""
        return self._next_is_right_arm_open

    @next_is_right_arm_open.setter
    def next_is_right_arm_open(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'next_is_right_arm_open' field must be of type 'bool'"
        self._next_is_right_arm_open = value


# Import statements for member types

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_Feedback(type):
    """Metaclass of message 'MechanismReset_Feedback'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_Feedback')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__feedback
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__feedback
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__feedback
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__feedback
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__feedback

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_Feedback(metaclass=Metaclass_MechanismReset_Feedback):
    """Message class 'MechanismReset_Feedback'."""

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_SendGoal_Request(type):
    """Metaclass of message 'MechanismReset_SendGoal_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_SendGoal_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__send_goal__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__send_goal__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__send_goal__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__send_goal__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__send_goal__request

            from tourobo_2026_interfaces.action import MechanismReset
            if MechanismReset.Goal.__class__._TYPE_SUPPORT is None:
                MechanismReset.Goal.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_SendGoal_Request(metaclass=Metaclass_MechanismReset_SendGoal_Request):
    """Message class 'MechanismReset_SendGoal_Request'."""

    __slots__ = [
        '_goal_id',
        '_goal',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'goal': 'tourobo_2026_interfaces/MechanismReset_Goal',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['tourobo_2026_interfaces', 'action'], 'MechanismReset_Goal'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Goal
        self.goal = kwargs.get('goal', MechanismReset_Goal())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.goal_id != other.goal_id:
            return False
        if self.goal != other.goal:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def goal(self):
        """Message field 'goal'."""
        return self._goal

    @goal.setter
    def goal(self, value):
        if __debug__:
            from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Goal
            assert \
                isinstance(value, MechanismReset_Goal), \
                "The 'goal' field must be a sub message of type 'MechanismReset_Goal'"
        self._goal = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_SendGoal_Response(type):
    """Metaclass of message 'MechanismReset_SendGoal_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_SendGoal_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__send_goal__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__send_goal__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__send_goal__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__send_goal__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__send_goal__response

            from builtin_interfaces.msg import Time
            if Time.__class__._TYPE_SUPPORT is None:
                Time.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_SendGoal_Response(metaclass=Metaclass_MechanismReset_SendGoal_Response):
    """Message class 'MechanismReset_SendGoal_Response'."""

    __slots__ = [
        '_accepted',
        '_stamp',
    ]

    _fields_and_field_types = {
        'accepted': 'boolean',
        'stamp': 'builtin_interfaces/Time',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.accepted = kwargs.get('accepted', bool())
        from builtin_interfaces.msg import Time
        self.stamp = kwargs.get('stamp', Time())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.accepted != other.accepted:
            return False
        if self.stamp != other.stamp:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def accepted(self):
        """Message field 'accepted'."""
        return self._accepted

    @accepted.setter
    def accepted(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'accepted' field must be of type 'bool'"
        self._accepted = value

    @builtins.property
    def stamp(self):
        """Message field 'stamp'."""
        return self._stamp

    @stamp.setter
    def stamp(self, value):
        if __debug__:
            from builtin_interfaces.msg import Time
            assert \
                isinstance(value, Time), \
                "The 'stamp' field must be a sub message of type 'Time'"
        self._stamp = value


class Metaclass_MechanismReset_SendGoal(type):
    """Metaclass of service 'MechanismReset_SendGoal'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_SendGoal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__mechanism_reset__send_goal

            from tourobo_2026_interfaces.action import _mechanism_reset
            if _mechanism_reset.Metaclass_MechanismReset_SendGoal_Request._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_SendGoal_Request.__import_type_support__()
            if _mechanism_reset.Metaclass_MechanismReset_SendGoal_Response._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_SendGoal_Response.__import_type_support__()


class MechanismReset_SendGoal(metaclass=Metaclass_MechanismReset_SendGoal):
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_SendGoal_Request as Request
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_SendGoal_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_GetResult_Request(type):
    """Metaclass of message 'MechanismReset_GetResult_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_GetResult_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__get_result__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__get_result__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__get_result__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__get_result__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__get_result__request

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_GetResult_Request(metaclass=Metaclass_MechanismReset_GetResult_Request):
    """Message class 'MechanismReset_GetResult_Request'."""

    __slots__ = [
        '_goal_id',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.goal_id != other.goal_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_GetResult_Response(type):
    """Metaclass of message 'MechanismReset_GetResult_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_GetResult_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__get_result__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__get_result__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__get_result__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__get_result__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__get_result__response

            from tourobo_2026_interfaces.action import MechanismReset
            if MechanismReset.Result.__class__._TYPE_SUPPORT is None:
                MechanismReset.Result.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_GetResult_Response(metaclass=Metaclass_MechanismReset_GetResult_Response):
    """Message class 'MechanismReset_GetResult_Response'."""

    __slots__ = [
        '_status',
        '_result',
    ]

    _fields_and_field_types = {
        'status': 'int8',
        'result': 'tourobo_2026_interfaces/MechanismReset_Result',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['tourobo_2026_interfaces', 'action'], 'MechanismReset_Result'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.status = kwargs.get('status', int())
        from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Result
        self.result = kwargs.get('result', MechanismReset_Result())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.status != other.status:
            return False
        if self.result != other.result:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def status(self):
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'status' field must be of type 'int'"
            assert value >= -128 and value < 128, \
                "The 'status' field must be an integer in [-128, 127]"
        self._status = value

    @builtins.property
    def result(self):
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value):
        if __debug__:
            from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Result
            assert \
                isinstance(value, MechanismReset_Result), \
                "The 'result' field must be a sub message of type 'MechanismReset_Result'"
        self._result = value


class Metaclass_MechanismReset_GetResult(type):
    """Metaclass of service 'MechanismReset_GetResult'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_GetResult')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__mechanism_reset__get_result

            from tourobo_2026_interfaces.action import _mechanism_reset
            if _mechanism_reset.Metaclass_MechanismReset_GetResult_Request._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_GetResult_Request.__import_type_support__()
            if _mechanism_reset.Metaclass_MechanismReset_GetResult_Response._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_GetResult_Response.__import_type_support__()


class MechanismReset_GetResult(metaclass=Metaclass_MechanismReset_GetResult):
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_GetResult_Request as Request
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_GetResult_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MechanismReset_FeedbackMessage(type):
    """Metaclass of message 'MechanismReset_FeedbackMessage'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset_FeedbackMessage')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mechanism_reset__feedback_message
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mechanism_reset__feedback_message
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mechanism_reset__feedback_message
            cls._TYPE_SUPPORT = module.type_support_msg__action__mechanism_reset__feedback_message
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mechanism_reset__feedback_message

            from tourobo_2026_interfaces.action import MechanismReset
            if MechanismReset.Feedback.__class__._TYPE_SUPPORT is None:
                MechanismReset.Feedback.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MechanismReset_FeedbackMessage(metaclass=Metaclass_MechanismReset_FeedbackMessage):
    """Message class 'MechanismReset_FeedbackMessage'."""

    __slots__ = [
        '_goal_id',
        '_feedback',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'feedback': 'tourobo_2026_interfaces/MechanismReset_Feedback',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['tourobo_2026_interfaces', 'action'], 'MechanismReset_Feedback'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Feedback
        self.feedback = kwargs.get('feedback', MechanismReset_Feedback())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.goal_id != other.goal_id:
            return False
        if self.feedback != other.feedback:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def feedback(self):
        """Message field 'feedback'."""
        return self._feedback

    @feedback.setter
    def feedback(self, value):
        if __debug__:
            from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Feedback
            assert \
                isinstance(value, MechanismReset_Feedback), \
                "The 'feedback' field must be a sub message of type 'MechanismReset_Feedback'"
        self._feedback = value


class Metaclass_MechanismReset(type):
    """Metaclass of action 'MechanismReset'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('tourobo_2026_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'tourobo_2026_interfaces.action.MechanismReset')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_action__action__mechanism_reset

            from action_msgs.msg import _goal_status_array
            if _goal_status_array.Metaclass_GoalStatusArray._TYPE_SUPPORT is None:
                _goal_status_array.Metaclass_GoalStatusArray.__import_type_support__()
            from action_msgs.srv import _cancel_goal
            if _cancel_goal.Metaclass_CancelGoal._TYPE_SUPPORT is None:
                _cancel_goal.Metaclass_CancelGoal.__import_type_support__()

            from tourobo_2026_interfaces.action import _mechanism_reset
            if _mechanism_reset.Metaclass_MechanismReset_SendGoal._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_SendGoal.__import_type_support__()
            if _mechanism_reset.Metaclass_MechanismReset_GetResult._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_GetResult.__import_type_support__()
            if _mechanism_reset.Metaclass_MechanismReset_FeedbackMessage._TYPE_SUPPORT is None:
                _mechanism_reset.Metaclass_MechanismReset_FeedbackMessage.__import_type_support__()


class MechanismReset(metaclass=Metaclass_MechanismReset):

    # The goal message defined in the action definition.
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Goal as Goal
    # The result message defined in the action definition.
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Result as Result
    # The feedback message defined in the action definition.
    from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_Feedback as Feedback

    class Impl:

        # The send_goal service using a wrapped version of the goal message as a request.
        from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_SendGoal as SendGoalService
        # The get_result service using a wrapped version of the result message as a response.
        from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_GetResult as GetResultService
        # The feedback message with generic fields which wraps the feedback message.
        from tourobo_2026_interfaces.action._mechanism_reset import MechanismReset_FeedbackMessage as FeedbackMessage

        # The generic service to cancel a goal.
        from action_msgs.srv._cancel_goal import CancelGoal as CancelGoalService
        # The generic message for get the status of a goal.
        from action_msgs.msg._goal_status_array import GoalStatusArray as GoalStatusMessage

    def __init__(self):
        raise NotImplementedError('Action classes can not be instantiated')
