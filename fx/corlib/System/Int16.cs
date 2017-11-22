namespace System
{
    public struct Int16
    {
        // The runtime sets this field automatically
        #pragma warning disable 0649
        private short _value;
        #pragma warning restore 0649

        public override bool Equals(object other)
        {
            if (other is Int16 i)
            {
                return i._value == _value;
            }
            else
            {
                return false;
            }
        }

        public override int GetHashCode()
        {
            return _value;
        }
    }
}