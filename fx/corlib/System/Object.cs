namespace System
{
    public class Object
    {
        public Object()
        {

        }

        public virtual string ToString()
        {
            return GetType().ToString();
        }

        public virtual Type GetType()
        {
            // No idea;
            return new Type();
        }

        public virtual bool Equals(object other)
        {
            // No idea
            return false;
        }

        public virtual int GetHashCode()
        {
            // The best hash code ever
            return 42;
        }
    }
}