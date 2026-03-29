document.addEventListener('DOMContentLoaded', () => {
    // 1. LOADER
    const loader = document.getElementById('loader');
    window.addEventListener('load', () => {
        setTimeout(() => {
            loader.style.opacity = '0';
            setTimeout(() => {
                loader.style.display = 'none';
            }, 500);
        }, 1000);
    });

    // 2. NAV SCROLL EFFECT
    const nav = document.getElementById('main-nav');
    window.addEventListener('scroll', () => {
        if (window.scrollY > 50) {
            nav.classList.add('scrolled');
        } else {
            nav.classList.remove('scrolled');
        }
    });

    // 3. REVEAL ANIMATIONS
    const revealElements = document.querySelectorAll('.reveal');
    const revealOnScroll = () => {
        const triggerBottom = window.innerHeight * 0.85;
        revealElements.forEach(el => {
            const elTop = el.getBoundingClientRect().top;
            if (elTop < triggerBottom) {
                el.classList.add('active');
            }
        });
    };
    window.addEventListener('scroll', revealOnScroll);
    revealOnScroll(); // Initial check

    // 4. PARALLAX EFFECT FOR HERO
    const heroBg = document.querySelector('.hero-bg');
    window.addEventListener('scroll', () => {
        const scroll = window.pageYOffset;
        if (heroBg) {
            heroBg.style.transform = `scale(1.1) translateY(${scroll * 0.3}px)`;
        }
    });

    // 5. SMOOTH SCROLLING FOR ANCHORS
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });

    // 6. DYNAMIC SYSTEM STATUS
    const statusItems = document.querySelectorAll('.status-item');
    setInterval(() => {
        const rand = Math.floor(Math.random() * statusItems.length);
        const item = statusItems[rand];
        item.style.opacity = '0.5';
        setTimeout(() => {
            item.style.opacity = '1';
        }, 300);
    }, 3000);

    // 7. MENU TOGGLE (MOBILE)
    const menuToggle = document.querySelector('.menu-toggle');
    const navLinks = document.querySelector('.nav-links');
    if (menuToggle) {
        menuToggle.addEventListener('click', () => {
            navLinks.style.display = navLinks.style.display === 'flex' ? 'none' : 'flex';
            navLinks.style.flexDirection = 'column';
            navLinks.style.position = 'absolute';
            navLinks.style.top = '80px';
            navLinks.style.left = '0';
            navLinks.style.width = '100%';
            navLinks.style.background = 'var(--bg-glass)';
            navLinks.style.padding = '20px';
            navLinks.style.backdropFilter = 'blur(15px)';
        });
    }
});
